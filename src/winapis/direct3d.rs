use super::{winapi::WindowsApplication, *};
use std::{fs::File, io::Read};
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

pub struct D3DApplication {
    context: ID3D11DeviceContext,
    swapchain: IDXGISwapChain,
    rtv_bbuf: Option<ID3D11RenderTargetView>,
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
        let (vshader, pshader, ilayout) = create_shader_coms(&device, winapp.get_cur_dir())?;
        // Set render configure
        unsafe { context.IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST) };
        unsafe { context.IASetInputLayout(ilayout) };
        unsafe { context.VSSetShader(vshader, std::ptr::null(), 0) };
        unsafe { context.PSSetShader(pshader, std::ptr::null(), 0) };
        // Finish
        Ok(Self {
            context,
            swapchain,
            rtv_bbuf: Some(rtv_bbuf),
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
}

/// A function for create coms around shader effectively.
fn create_shader_coms(
    device: &ID3D11Device,
    dir: &String,
) -> Result<(ID3D11VertexShader, ID3D11PixelShader, ID3D11InputLayout), MyErr> {
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
    Ok((vshader, pshader, ilayout))
}
