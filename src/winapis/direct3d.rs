use super::{winapi::WindowsApplication, *};
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::*,
        Graphics::{
            Direct3D::{Fxc::*, *},
            Direct3D11::*,
            Dxgi::{Common::*, *},
        },
    },
};

pub struct D3DApplication {
    rtv_bbuf: ID3D11RenderTargetView,
}

impl D3DApplication {
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
        let path = PCWSTR(
            (winapp.get_current_path().clone() + "shaders.hlsl\0")
                .encode_utf16()
                .collect::<Vec<u16>>()
                .as_ptr(),
        );
        let vshader = unsafe {
            let mut vshader_mut = None;
            D3DCompileFromFile(
                path,
                std::ptr::null(),
                None,
                "VSMain",
                "vs_5_0",
                0,
                0,
                &mut vshader_mut,
                std::ptr::null_mut(),
            )
            .map_err(|_| MyErr::D3DApp(ErrKnd::IO, String::from("Compilation vshader failed")))?;
            vshader_mut.ok_or(MyErr::D3DApp(ErrKnd::Creation, String::from("vshader")))?
        };
        let pshader = unsafe {
            let mut pshader_mut = None;
            D3DCompileFromFile(
                path,
                std::ptr::null(),
                None,
                "PSMain",
                "ps_5_0",
                0,
                0,
                &mut pshader_mut,
                std::ptr::null_mut(),
            )
            .map_err(|_| MyErr::D3DApp(ErrKnd::IO, String::from("Compilation pshader failed")))?;
            pshader_mut.ok_or(MyErr::D3DApp(ErrKnd::Creation, String::from("pshader")))?
        };
        unsafe { context.VSSetShader(vshader, std::ptr::null(), 0) };
        Ok(Self { rtv_bbuf })
    }
}
