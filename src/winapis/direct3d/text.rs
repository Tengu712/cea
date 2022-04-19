use super::{super::winapi::WindowsApplication, *};
use windows::{
    core::{Interface, PCWSTR},
    Foundation::Numerics::*,
    Win32::{
        Graphics::{
            Direct2D::{Common::*, *},
            DirectWrite::*,
            Dxgi::{Common::*, *},
        },
        UI::HiDpi::GetDpiForWindow,
    },
};

pub enum TextAlign {
    Left,
    Center,
    Right,
}

/// Use it when drawing text.
pub struct DrawingTextDesc {
    pub text: String,
    pub font: String,
    pub size: f32,
    pub rect: [f32; 4],
    pub rgba: [f32; 4],
    pub align: TextAlign,
}

pub struct D3DTextModule {
    d2context: ID2D1DeviceContext,
    dwfactory: IDWriteFactory,
    bitmap: ID2D1Bitmap1,
}
impl D3DTextModule {
    pub fn draw_text(&self, desc: &DrawingTextDesc) -> Result<(), MyErr> {
        let format = unsafe {
            self.dwfactory
                .CreateTextFormat(
                    PCWSTR(desc.font.encode_utf16().collect::<Vec<u16>>().as_ptr()),
                    None,
                    DWRITE_FONT_WEIGHT_NORMAL,
                    DWRITE_FONT_STYLE_NORMAL,
                    DWRITE_FONT_STRETCH_NORMAL,
                    desc.size * 72.0 / 96.0,
                    PCWSTR("ja-JP\0".encode_utf16().collect::<Vec<u16>>().as_ptr()),
                )
                .map_err(|_| MyErr::d3d(EKnd::Runtime, "Creation text format failed"))?
            };
            let alignment = match desc.align {
                TextAlign::Left => DWRITE_TEXT_ALIGNMENT_LEADING,
                TextAlign::Center => DWRITE_TEXT_ALIGNMENT_CENTER,
                TextAlign::Right => DWRITE_TEXT_ALIGNMENT_TRAILING,
            };
            unsafe { format.SetTextAlignment(alignment)
                .map_err(|_| MyErr::d3d(EKnd::Runtime, "Set text alignment"))?
         };
        let brush = unsafe {
            let color = D2D1_COLOR_F {
                r: desc.rgba[0],
                g: desc.rgba[1],
                b: desc.rgba[2],
                a: desc.rgba[3],
            };
            let brushproperties = D2D1_BRUSH_PROPERTIES {
                opacity: 1.0,
                transform: Matrix3x2::identity(),
            };
            self.d2context
                .CreateSolidColorBrush(&color, &brushproperties)
                .map_err(|_| MyErr::d3d(EKnd::Runtime, "Creation text brush failed"))?
        };
        let layoutrect = D2D_RECT_F {
            left: desc.rect[0],
            right: desc.rect[1],
            top: desc.rect[2],
            bottom: desc.rect[3],
        };
        let v_text = desc.text.encode_utf16().collect::<Vec<u16>>();
        unsafe {
            self.d2context.BeginDraw();
            self.d2context.SetTarget(&self.bitmap);
            self.d2context.DrawText(
                PCWSTR(v_text.as_ptr()),
                v_text.len() as u32,
                &format,
                &layoutrect,
                &brush,
                D2D1_DRAW_TEXT_OPTIONS_NONE,
                DWRITE_MEASURING_MODE_NATURAL,
            );
            self.d2context
                .EndDraw(std::ptr::null_mut(), std::ptr::null_mut())
                .map_err(|e| MyErr::d3d(EKnd::Runtime, e.to_string().as_str()))?;
        };
        Ok(())
    }
}

impl D3DApplication {
    /// Create DirectWrite coms based on Direct3D coms. 
    pub fn create_text_module(&self, winapp: &WindowsApplication) -> Result<D3DTextModule, MyErr> {
        // Create factory first of all
        let factory: ID2D1Factory3 = unsafe {
            let mut ppifactory = None;
            D2D1CreateFactory(
                D2D1_FACTORY_TYPE_SINGLE_THREADED,
                &ID2D1Factory3::IID,
                &D2D1_FACTORY_OPTIONS::default(),
                std::mem::transmute(&mut ppifactory),
            )
            .map_err(|_| MyErr::d3d(EKnd::Creation, "ID2D1Factory for DWrite"))?;
            ppifactory.ok_or(MyErr::d3d(EKnd::Common, "ID2D1Factory for DWrite is none"))?
        };
        // Create d2context from factory
        let d2context = unsafe {
            let dxdevice = self
                .device
                .cast::<IDXGIDevice>()
                .map_err(|_| MyErr::d3d(EKnd::Creation, "IDXGIDevice for DWrite"))?;
            factory
                .CreateDevice(&dxdevice)
                .map_err(|_| MyErr::d3d(EKnd::Creation, "ID2D1Device for DWrite"))?
                .CreateDeviceContext(D2D1_DEVICE_CONTEXT_OPTIONS_NONE)
                .map_err(|_| MyErr::d3d(EKnd::Creation, "ID2D1DeviceContext for DWrite"))?
        };
        // Create bitmap from d2context
        let bitmap = unsafe {
            let dpi = GetDpiForWindow(*winapp.get_window_handle());
            let backbuffer = self
                .swapchain
                .GetBuffer::<IDXGISurface>(0)
                .map_err(|_| MyErr::d3d(EKnd::Get, "backbuffer for DWrite"))?;
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
                .map_err(|_| MyErr::d3d(EKnd::Creation, "Bitmap for DWrite"))?
        };
        // Create dwfactory
        let dwfactory = unsafe {
            DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED, &IDWriteFactory::IID)
                .map_err(|_| MyErr::d3d(EKnd::Creation, "DWrite factory"))?
        };
        let dwfactory = unsafe { std::mem::transmute(dwfactory) };
        // Finish
        Ok(D3DTextModule {
            d2context,
            dwfactory,
            bitmap,
        })
    }
}
