use super::{super::winapi::WindowsApplication, shader::ShaderComs, D3DApplication};
use windows::{
    core::{Error, Result, HRESULT, HSTRING},
    Win32::{
        Foundation::{BOOL, HINSTANCE},
        Graphics::{
            Direct3D::{
                D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST, D3D_DRIVER_TYPE_HARDWARE,
                D3D_FEATURE_LEVEL_11_1,
            },
            Direct3D11::*,
            Dxgi::{Common::*, *},
        },
    },
};

impl D3DApplication {
    pub fn new(
        winapp: &WindowsApplication,
        width: u32,
        height: u32,
        shader_dir: &str,
    ) -> Result<Self> {
        // Create factory
        let factory = unsafe { CreateDXGIFactory::<IDXGIFactory>()? };
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
            )?;
            (
                ppdevice.ok_or(Error::new(
                    HRESULT(0x80004005u32 as i32),
                    HSTRING::from("Failed to create D3D11Device."),
                ))?,
                ppimmediatecontext.ok_or(Error::new(
                    HRESULT(0x80004005u32 as i32),
                    HSTRING::from("Failed to create D3D11DeviceContext."),
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
            factory.CreateSwapChain(&device, &pdesc)?
        };
        // Create back buffer rtv
        let rtv_bbuf = unsafe {
            let bbuf = swapchain.GetBuffer::<ID3D11Texture2D>(0)?;
            device.CreateRenderTargetView(bbuf, std::ptr::null())?
        };
        // Create Depth stencil buffer
        let dsview = unsafe {
            let dstex = {
                let desc = D3D11_TEXTURE2D_DESC {
                    Width: width,
                    Height: height,
                    MipLevels: 1,
                    ArraySize: 1,
                    Format: DXGI_FORMAT_R24G8_TYPELESS,
                    SampleDesc: DXGI_SAMPLE_DESC {
                        Count: 1,
                        Quality: 0,
                    },
                    Usage: D3D11_USAGE_DEFAULT,
                    BindFlags: D3D11_BIND_DEPTH_STENCIL | D3D11_BIND_SHADER_RESOURCE,
                    CPUAccessFlags: D3D11_CPU_ACCESS_FLAG(0),
                    MiscFlags: D3D11_RESOURCE_MISC_FLAG(0),
                };
                device.CreateTexture2D(&desc, std::ptr::null())?
            };
            let desc = D3D11_DEPTH_STENCIL_VIEW_DESC {
                Format: DXGI_FORMAT_D24_UNORM_S8_UINT,
                ViewDimension: D3D11_DSV_DIMENSION_TEXTURE2D,
                Flags: 0,
                Anonymous: D3D11_DEPTH_STENCIL_VIEW_DESC_0 {
                    Texture2D: D3D11_TEX2D_DSV { MipSlice: 0 },
                },
            };
            device.CreateDepthStencilView(&dstex, &desc)?
        };
        let dsstate = unsafe {
            let desc = D3D11_DEPTH_STENCIL_DESC {
                DepthEnable: BOOL(1),
                DepthWriteMask: D3D11_DEPTH_WRITE_MASK_ALL,
                DepthFunc: D3D11_COMPARISON_LESS_EQUAL,
                StencilEnable: BOOL(0),
                ..Default::default()
            };
            device.CreateDepthStencilState(&desc)?
        };
        unsafe { context.OMSetDepthStencilState(&dsstate, 0) };
        // Create shaders
        let shader_coms = ShaderComs::new(&device, &shader_dir.to_string())?;
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
        let blend_state = unsafe { device.CreateBlendState(&blend_desc)? };
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
            dsview,
            cbuffer: Some(shader_coms.cbuffer),
        })
    }
    /// Set render target view.
    pub fn set_rtv(&self) {
        unsafe {
            self.context
                .OMSetRenderTargets(1, &self.rtv_bbuf, &self.dsview)
        };
    }
    /// Clear render target view.
    pub fn clear_rtv(&self) {
        unsafe {
            self.context
                .ClearRenderTargetView(&self.rtv_bbuf, [0.0, 0.0, 0.0, 1.0].as_ptr());
            self.context.ClearDepthStencilView(&self.dsview, 1 | 2, 1.0, 0);
        };
    }
    /// Swap and wait vsync.
    pub fn swap(&self) -> Result<()> {
        unsafe { self.swapchain.Present(1, 0) }
    }
}
