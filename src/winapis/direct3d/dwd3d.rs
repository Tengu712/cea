use super::{
    super::{directwrite::DWriteApp, winapi::WindowsApplication},
    D3DApplication,
};
use windows::{
    core::{Error, Interface, Result, HRESULT, HSTRING},
    Win32::{
        Graphics::{
            Direct2D::{
                Common::{D2D1_ALPHA_MODE_PREMULTIPLIED, D2D1_PIXEL_FORMAT},
                *,
            },
            DirectWrite::{DWriteCreateFactory, IDWriteFactory5, DWRITE_FACTORY_TYPE_SHARED},
            Dxgi::{Common::DXGI_FORMAT_R8G8B8A8_UNORM, IDXGIDevice, IDXGISurface},
        },
        UI::HiDpi::GetDpiForWindow,
    },
};

impl D3DApplication {
    /// Create DirectWrite coms based on Direct3D coms.
    pub fn create_text_module(&self, winapp: &WindowsApplication) -> Result<DWriteApp> {
        // Create factory first of all
        let factory: ID2D1Factory3 = unsafe {
            let mut ppifactory = None;
            D2D1CreateFactory(
                D2D1_FACTORY_TYPE_SINGLE_THREADED,
                &ID2D1Factory3::IID,
                &D2D1_FACTORY_OPTIONS::default(),
                std::mem::transmute(&mut ppifactory),
            )?;
            ppifactory.ok_or(Error::new(
                HRESULT(0x80004005u32 as i32),
                HSTRING::from("Failed to create ID2D1Factory for DirectWrite."),
            ))?
        };
        // Create d2context from factory
        let d2context = unsafe {
            let dxdevice = self.device.cast::<IDXGIDevice>()?;
            factory
                .CreateDevice(&dxdevice)?
                .CreateDeviceContext(D2D1_DEVICE_CONTEXT_OPTIONS_NONE)?
        };
        // Create bitmap from d2context
        let bitmap = unsafe {
            let dpi = GetDpiForWindow(*winapp.get_window_handle());
            let backbuffer = self.swapchain.GetBuffer::<IDXGISurface>(0)?;
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
            d2context.CreateBitmapFromDxgiSurface(&backbuffer, &bitmapproperties)?
        };
        // Create dwfactory
        let dwfactory =
            unsafe { DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED, &IDWriteFactory5::IID)? };
        let dwfactory: IDWriteFactory5 = unsafe { std::mem::transmute(dwfactory) };
        // Finish
        Ok(DWriteApp::from(d2context, dwfactory, bitmap))
    }
}
