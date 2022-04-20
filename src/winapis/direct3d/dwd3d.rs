use super::{super::directwrite::DWriteApp, super::winapi::WindowsApplication, *};
use windows::{
    core::Interface,
    Win32::{
        Graphics::{
            Direct2D::{Common::*, *},
            DirectWrite::*,
            Dxgi::{Common::*, *},
        },
        UI::HiDpi::GetDpiForWindow,
    },
};

impl D3DApplication {
    /// Create DirectWrite coms based on Direct3D coms.
    pub fn create_text_module(&self, winapp: &WindowsApplication) -> Result<DWriteApp, WErr> {
        // Create factory first of all
        let factory: ID2D1Factory3 = unsafe {
            let mut ppifactory = None;
            D2D1CreateFactory(
                D2D1_FACTORY_TYPE_SINGLE_THREADED,
                &ID2D1Factory3::IID,
                &D2D1_FACTORY_OPTIONS::default(),
                std::mem::transmute(&mut ppifactory),
            )
            .map_err(|_| raise_err(EKnd::Creation, "ID2D1Factory for DWrite"))?;
            ppifactory.ok_or(raise_err(EKnd::Common, "ID2D1Factory for DWrite is none"))?
        };
        // Create d2context from factory
        let d2context = unsafe {
            let dxdevice = self
                .device
                .cast::<IDXGIDevice>()
                .map_err(|_| raise_err(EKnd::Creation, "IDXGIDevice for DWrite"))?;
            factory
                .CreateDevice(&dxdevice)
                .map_err(|_| raise_err(EKnd::Creation, "ID2D1Device for DWrite"))?
                .CreateDeviceContext(D2D1_DEVICE_CONTEXT_OPTIONS_NONE)
                .map_err(|_| raise_err(EKnd::Creation, "ID2D1DeviceContext for DWrite"))?
        };
        // Create bitmap from d2context
        let bitmap = unsafe {
            let dpi = GetDpiForWindow(*winapp.get_window_handle());
            let backbuffer = self
                .swapchain
                .GetBuffer::<IDXGISurface>(0)
                .map_err(|_| raise_err(EKnd::Get, "backbuffer for DWrite"))?;
            let bitmapproperties = D2D1_BITMAP_PROPERTIES1 {
                pixelFormat: D2D1_PIXEL_FORMAT {
                    format: DXGI_FORMAT_R8G8B8A8_UNORM,
                    alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
                },
                dpiX: dpi as f32,
                dpiY: dpi as f32,
                bitmapOptions: D2D1_BITMAP_OPTIONS_TARGET | D2D1_BITMAP_OPTIONS_CANNOT_DRAW,
                colorContext: None,
            };
            d2context
                .CreateBitmapFromDxgiSurface(&backbuffer, &bitmapproperties)
                .map_err(|_| raise_err(EKnd::Creation, "Bitmap for DWrite"))?
        };
        // Create dwfactory
        let dwfactory = unsafe {
            DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED, &IDWriteFactory5::IID)
                .map_err(|_| raise_err(EKnd::Creation, "DWrite factory"))?
        };
        let dwfactory: IDWriteFactory5 = unsafe { std::mem::transmute(dwfactory) };
        // Finish
        Ok(DWriteApp::from(d2context, dwfactory, bitmap))
    }
}
