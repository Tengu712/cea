use super::{math::Matrix4x4, winapi::WindowsApplication, *};
use std::{fs::File, io::Read, mem::size_of};
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::*,
        Graphics::{
            Direct3D::*,
            Direct3D11::*,
            Dxgi::{Common::*, *},
        },
    },
};

pub struct Vertex {
    pub pos: [f32; 3],
    pub col: [f32; 4],
    pub tex: [f32; 2],
}

pub struct CData {
    pub mat_scl: Matrix4x4,
    pub mat_rtx: Matrix4x4,
    pub mat_rty: Matrix4x4,
    pub mat_rtz: Matrix4x4,
    pub mat_trs: Matrix4x4,
    pub mat_view: Matrix4x4,
    pub mat_proj: Matrix4x4,
    pub vec_col: [f32; 4],
    pub vec_prm: [f32; 4],
}

pub struct ModelBuffer {
    num_idx: u32,
    vbuf: Option<ID3D11Buffer>,
    ibuf: Option<ID3D11Buffer>,
}

pub struct D3DApplication {
    device: ID3D11Device,
    context: ID3D11DeviceContext,
    swapchain: IDXGISwapChain,
    rtv_bbuf: Option<ID3D11RenderTargetView>,
    cbuffer: Option<ID3D11Buffer>,
}
impl D3DApplication {
    /// Constructor.
    pub fn new(winapp: &WindowsApplication, width: u32, height: u32) -> Result<Self, MyErr> {
        // Create factory
        let factory = unsafe {
            CreateDXGIFactory::<IDXGIFactory>()
                .map_err(|_| MyErr::D3DApp(ErrKnd::Creation, String::from("DXGIFactory")))?
        };
        // Create device
        let (device, context) = unsafe {
            let mut ppdevice = None;
            let mut ppimmediatecontext = None;
            D3D11CreateDevice(
                None,
                D3D_DRIVER_TYPE_HARDWARE,
                HINSTANCE::default(),
                D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                &D3D_FEATURE_LEVEL_11_1,
                1,
                D3D11_SDK_VERSION,
                &mut ppdevice,
                std::ptr::null_mut(),
                &mut ppimmediatecontext,
            )
            .map_err(|_| MyErr::D3DApp(ErrKnd::Common, String::from("D3D11CreateDevice failed")))?;
            (
                ppdevice.ok_or(MyErr::D3DApp(ErrKnd::Creation, String::from("D3D11Device")))?,
                ppimmediatecontext.ok_or(MyErr::D3DApp(
                    ErrKnd::Creation,
                    String::from("D3D11DeviceContext"),
                ))?,
            )
        };
        // Create swapchain
        let swapchain = unsafe {
            let buffer_desc = DXGI_MODE_DESC {
                Width: width,
                Height: height,
                RefreshRate: DXGI_RATIONAL {
                    Numerator: 60,
                    Denominator: 1,
                },
                Format: DXGI_FORMAT_R8G8B8A8_UNORM,
                ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
                Scaling: DXGI_MODE_SCALING_UNSPECIFIED,
            };
            let pdesc = DXGI_SWAP_CHAIN_DESC {
                BufferDesc: buffer_desc,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
                BufferCount: 1,
                OutputWindow: *winapp.get_window_handle(),
                Windowed: BOOL(1),
                SwapEffect: DXGI_SWAP_EFFECT_DISCARD,
                Flags: 2,
            };
            factory
                .CreateSwapChain(&device, &pdesc)
                .map_err(|_| MyErr::D3DApp(ErrKnd::Creation, String::from("SwapChain")))?
        };
        // Create back buffer rtv
        let rtv_bbuf = unsafe {
            let bbuf = swapchain
                .GetBuffer::<ID3D11Texture2D>(0)
                .map_err(|_| MyErr::D3DApp(ErrKnd::Get, String::from("RTV of backbuffer")))?;
            device
                .CreateRenderTargetView(bbuf, std::ptr::null())
                .map_err(|_| MyErr::D3DApp(ErrKnd::Creation, String::from("RTV of backbuffer")))?
        };
        // Create shaders
        let shader_coms = ShaderComs::new(&device, winapp.get_cur_dir())?;
        // Set render configure
        let viewport = D3D11_VIEWPORT {
            TopLeftX: 0.0,
            TopLeftY: 0.0,
            Width: width as f32,
            Height: height as f32,
            MinDepth: 0.0,
            MaxDepth: 1.0,
        };
        unsafe { context.RSSetViewports(1, &viewport) };
        let blend_desc = {
            let mut render_targets = [D3D11_RENDER_TARGET_BLEND_DESC::default(); 8];
            render_targets[0] = D3D11_RENDER_TARGET_BLEND_DESC {
                BlendEnable: BOOL(1),
                SrcBlend: D3D11_BLEND_SRC_ALPHA,
                DestBlend: D3D11_BLEND_INV_SRC_ALPHA,
                BlendOp: D3D11_BLEND_OP_ADD,
                SrcBlendAlpha: D3D11_BLEND_ONE,
                DestBlendAlpha: D3D11_BLEND_ONE,
                BlendOpAlpha: D3D11_BLEND_OP_ADD,
                RenderTargetWriteMask: 0b1111,
            };
            D3D11_BLEND_DESC {
                AlphaToCoverageEnable: BOOL(0),
                IndependentBlendEnable: BOOL(0),
                RenderTarget: render_targets,
            }
        };
        let blend_state = unsafe {
            device
                .CreateBlendState(&blend_desc)
                .map_err(|_| MyErr::D3DApp(ErrKnd::Creation, String::from("BlendState")))?
        };
        unsafe { context.OMSetBlendState(blend_state, [1.0, 1.0, 1.0, 1.0].as_ptr(), 0xffffffff) };
        unsafe { context.IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST) };
        unsafe { context.IASetInputLayout(shader_coms.ilayout) };
        unsafe { context.VSSetShader(shader_coms.vshader, std::ptr::null(), 0) };
        unsafe { context.PSSetShader(shader_coms.pshader, std::ptr::null(), 0) };
        // Finish
        Ok(Self {
            device,
            context,
            swapchain,
            rtv_bbuf: Some(rtv_bbuf),
            cbuffer: Some(shader_coms.cbuffer),
        })
    }
    /// Create model buffer.
    pub fn create_modelbuffer(
        &self,
        num_vtx: u32,
        data_vtx: &[Vertex],
        num_idx: u32,
        data_idx: &[u32],
    ) -> Result<ModelBuffer, MyErr> {
        let vbuf = unsafe {
            let vbuf_desc = D3D11_BUFFER_DESC {
                ByteWidth: size_of::<Vertex>() as u32 * num_vtx,
                Usage: D3D11_USAGE_DEFAULT,
                BindFlags: 1,
                CPUAccessFlags: 0,
                MiscFlags: 0,
                StructureByteStride: 0,
            };
            let vbuf_data = D3D11_SUBRESOURCE_DATA {
                pSysMem: data_vtx as *const _ as *const ::core::ffi::c_void,
                SysMemPitch: 0,
                SysMemSlicePitch: 0,
            };
            self.device
                .CreateBuffer(&vbuf_desc, &vbuf_data)
                .map_err(|_| MyErr::D3DApp(ErrKnd::Creation, String::from("Model vbuffer")))?
        };
        let ibuf = unsafe {
            let ibuf_desc = D3D11_BUFFER_DESC {
                ByteWidth: size_of::<u32>() as u32 * num_idx,
                Usage: D3D11_USAGE_DEFAULT,
                BindFlags: 2,
                CPUAccessFlags: 0,
                MiscFlags: 0,
                StructureByteStride: 0,
            };
            let ibuf_data = D3D11_SUBRESOURCE_DATA {
                pSysMem: data_idx as *const _ as *const ::core::ffi::c_void,
                SysMemPitch: 0,
                SysMemSlicePitch: 0,
            };
            self.device
                .CreateBuffer(&ibuf_desc, &ibuf_data)
                .map_err(|_| MyErr::D3DApp(ErrKnd::Creation, String::from("Model ibuffer")))?
        };
        Ok(ModelBuffer {
            num_idx,
            vbuf: Some(vbuf),
            ibuf: Some(ibuf),
        })
    }
    /// Set render target view.
    pub fn set_rtv(&self) {
        unsafe { self.context.OMSetRenderTargets(1, &self.rtv_bbuf, None) };
    }
    /// Clear render target view.
    pub fn clear_rtv(&self) {
        unsafe {
            self.context
                .ClearRenderTargetView(&self.rtv_bbuf, [0.0, 0.0, 0.0, 1.0].as_ptr())
        };
    }
    /// Swap and wait vsync.
    pub fn swap(&self) -> Result<(), MyErr> {
        unsafe {
            self.swapchain
                .Present(1, 0)
                .map_err(|_| MyErr::D3DApp(ErrKnd::Runtime, String::from("Backbuffer swap failed")))
        }
    }
    /// Draw model.
    pub fn draw_model(&self, mbuf: &ModelBuffer, cdata: &CData) -> Result<(), MyErr> {
        unsafe {
            self.context
                .IASetVertexBuffers(0, 1, &mbuf.vbuf, &(size_of::<Vertex>() as u32), &0);
            self.context
                .IASetIndexBuffer(&mbuf.ibuf, DXGI_FORMAT_R32_UINT, 0);
            self.context.UpdateSubresource(
                self.cbuffer.as_ref().ok_or(MyErr::D3DApp(
                    ErrKnd::Runtime,
                    String::from("Cbuffer is None"),
                ))?,
                0,
                std::ptr::null(),
                cdata as *const _ as *const ::core::ffi::c_void,
                0,
                0,
            );
            self.context.VSSetConstantBuffers(0, 1, &self.cbuffer);
            self.context.DrawIndexed(mbuf.num_idx, 0, 0);
        };
        Ok(())
    }
}

struct ShaderComs {
    vshader: ID3D11VertexShader,
    pshader: ID3D11PixelShader,
    ilayout: ID3D11InputLayout,
    cbuffer: ID3D11Buffer,
}
impl ShaderComs {
    /// A function for create coms around shader effectively.
    fn new(device: &ID3D11Device, dir: &String) -> Result<Self, MyErr> {
        // Open vshader.cso here for input layout creation
        let mut vshader_bytebuf = Vec::new();
        File::open(dir.clone() + "vshader.cso")
            .map_err(|_| MyErr::D3DApp(ErrKnd::Io, String::from("Open vshader.cso failed")))?
            .read_to_end(&mut vshader_bytebuf)
            .map_err(|_| MyErr::D3DApp(ErrKnd::Io, String::from("Read vshader.cso failed")))?;
        let vshader_bytecode = vshader_bytebuf.as_ptr() as *const _ as *const ::core::ffi::c_void;
        // Vertex shader
        let vshader = unsafe {
            device
                .CreateVertexShader(vshader_bytecode, vshader_bytebuf.len(), None)
                .map_err(|_| MyErr::D3DApp(ErrKnd::Creation, String::from("vshader")))?
        };
        // Pixel shader
        let pshader = unsafe {
            let mut buf = Vec::new();
            File::open(dir.clone() + "pshader.cso")
                .map_err(|_| MyErr::D3DApp(ErrKnd::Io, String::from("Open pshader.cso failed")))?
                .read_to_end(&mut buf)
                .map_err(|_| MyErr::D3DApp(ErrKnd::Io, String::from("Read pshader.cso failed")))?;
            let bytecode = buf.as_ptr() as *const _ as *const ::core::ffi::c_void;
            device
                .CreatePixelShader(bytecode, buf.len(), None)
                .map_err(|_| MyErr::D3DApp(ErrKnd::Creation, String::from("pshader")))?
        };
        // Input layout
        let ilayout = unsafe {
            let pinputelementdescs = [
                D3D11_INPUT_ELEMENT_DESC {
                    SemanticName: PCSTR("POSITION\0".as_ptr()),
                    SemanticIndex: 0,
                    Format: DXGI_FORMAT_R32G32B32_FLOAT,
                    InputSlot: 0,
                    AlignedByteOffset: 0,
                    InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                    InstanceDataStepRate: 0,
                },
                D3D11_INPUT_ELEMENT_DESC {
                    SemanticName: PCSTR("COLOR\0".as_ptr()),
                    SemanticIndex: 0,
                    Format: DXGI_FORMAT_R32G32B32A32_FLOAT,
                    InputSlot: 0,
                    AlignedByteOffset: D3D11_APPEND_ALIGNED_ELEMENT,
                    InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                    InstanceDataStepRate: 0,
                },
                D3D11_INPUT_ELEMENT_DESC {
                    SemanticName: PCSTR("TEXCOORD\0".as_ptr()),
                    SemanticIndex: 0,
                    Format: DXGI_FORMAT_R32G32_FLOAT,
                    InputSlot: 0,
                    AlignedByteOffset: D3D11_APPEND_ALIGNED_ELEMENT,
                    InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                    InstanceDataStepRate: 0,
                },
            ];
            device
                .CreateInputLayout(
                    pinputelementdescs.as_ptr(),
                    pinputelementdescs.len() as u32,
                    vshader_bytecode,
                    vshader_bytebuf.len(),
                )
                .map_err(|_| MyErr::D3DApp(ErrKnd::Creation, String::from("InputLayout")))?
        };
        // Constant buffer
        let cbuffer = unsafe {
            let cbuf_desc = D3D11_BUFFER_DESC {
                ByteWidth: size_of::<CData>() as u32,
                Usage: D3D11_USAGE_DEFAULT,
                BindFlags: 4,
                CPUAccessFlags: 0,
                MiscFlags: 0,
                StructureByteStride: 0,
            };
            device
                .CreateBuffer(&cbuf_desc, std::ptr::null())
                .map_err(|_| MyErr::D3DApp(ErrKnd::Creation, String::from("Cbuffer")))?
        };
        Ok(Self {
            vshader,
            pshader,
            ilayout,
            cbuffer,
        })
    }
}
