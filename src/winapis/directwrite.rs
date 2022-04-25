use super::*;
use windows::{
    core::PCWSTR,
    Foundation::Numerics::Matrix3x2,
    Win32::Graphics::{
        Direct2D::{
            Common::{D2D1_COLOR_F, D2D_RECT_F},
            ID2D1Bitmap1, ID2D1DeviceContext, D2D1_BRUSH_PROPERTIES, D2D1_DRAW_TEXT_OPTIONS_NONE,
        },
        DirectWrite::*,
    },
};

fn raise_err(errknd: EKnd, message: &str) -> WErr {
    WErr::from(
        errknd,
        String::from(message),
        String::from("DirectWrite App"),
    )
}

/// Use it when drawing text.
pub struct TextDesc {
    text: String,
    rect: [f32; 4],
    rgba: [f32; 4],
    align: u32,
}
impl TextDesc {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            rect: [0.0, 1280.0, 0.0, 720.0],
            rgba: [1.0; 4],
            align: 0,
        }
    }
    pub fn set_text<T: std::string::ToString>(self, text: T) -> Self {
        let mut self_mut = self;
        self_mut.text = text.to_string();
        self_mut
    }
    pub fn set_rect(self, rect: [f32; 4]) -> Self {
        let mut self_mut = self;
        self_mut.rect = rect;
        self_mut
    }
    pub fn set_rgba(self, rgba: [f32; 4]) -> Self {
        let mut self_mut = self;
        self_mut.rgba = rgba;
        self_mut
    }
    pub fn set_align(self, align: u32) -> Self {
        let mut self_mut = self;
        self_mut.align = align;
        self_mut
    }
}

pub struct DWriteApp {
    d2context: ID2D1DeviceContext,
    dwfactory: IDWriteFactory5,
    bitmap: ID2D1Bitmap1,
}
impl DWriteApp {
    pub(super) fn from(
        d2context: ID2D1DeviceContext,
        dwfactory: IDWriteFactory5,
        bitmap: ID2D1Bitmap1,
    ) -> Self {
        Self {
            d2context,
            dwfactory,
            bitmap,
        }
    }
    /// Draw text. To call it, user give it DrawTextDesc struct.
    pub fn draw_text(&self, desc: &TextDesc, format: &IDWriteTextFormat) -> Result<(), WErr> {
        let alignment = if desc.align == 1 {
            DWRITE_TEXT_ALIGNMENT_CENTER
        } else if desc.align == 2 {
            DWRITE_TEXT_ALIGNMENT_TRAILING
        } else {
            DWRITE_TEXT_ALIGNMENT_LEADING
        };
        unsafe {
            format
                .SetTextAlignment(alignment)
                .map_err(|_| raise_err(EKnd::Runtime, "Set text alignment"))?
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
                .map_err(|_| raise_err(EKnd::Runtime, "Creation text brush failed"))?
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
                format,
                &layoutrect,
                &brush,
                D2D1_DRAW_TEXT_OPTIONS_NONE,
                DWRITE_MEASURING_MODE_NATURAL,
            );
            self.d2context
                .EndDraw(std::ptr::null_mut(), std::ptr::null_mut())
                .map_err(|e| raise_err(EKnd::Runtime, e.to_string().as_str()))?;
        };
        Ok(())
    }
    /// Create text format. If no custom font, fontcollection must be None.
    pub fn create_text_format<'a, T>(
        &self,
        font: &str,
        fontcollection: T,
        size: f32,
    ) -> Result<IDWriteTextFormat, WErr>
    where
        T: ::windows::core::IntoParam<'a, IDWriteFontCollection>,
    {
        unsafe {
            self.dwfactory.CreateTextFormat(
                PCWSTR(font.encode_utf16().collect::<Vec<u16>>().as_ptr()),
                fontcollection,
                DWRITE_FONT_WEIGHT_NORMAL,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                size * 72.0 / 96.0,
                PCWSTR("ja-JP\0".encode_utf16().collect::<Vec<u16>>().as_ptr()),
            )
        }
        .map_err(|_| raise_err(EKnd::Runtime, "Creation text format failed"))
    }
}
