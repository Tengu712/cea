use windows::{
    core::{Result, PCWSTR},
    Foundation::Numerics::Matrix3x2,
    Win32::Graphics::{
        Direct2D::{
            Common::{D2D1_COLOR_F, D2D_RECT_F},
            ID2D1Bitmap1, ID2D1DeviceContext, D2D1_BRUSH_PROPERTIES, D2D1_DRAW_TEXT_OPTIONS_NONE,
        },
        DirectWrite::*,
    },
};

/// Use it when drawing text.
pub struct TextDesc {
    pub text: String,
    pub rect: [f32; 4],
    pub rgba: [f32; 4],
}
pub struct TextFormatDesc {
    pub fontname: &'static str,
    pub size: f32,
    pub align: u32,
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
    pub fn draw_text<'a, T>(
        &self,
        desc: &TextDesc,
        formatdesc: &TextFormatDesc,
        fontcollection: T,
    ) -> Result<()>
    where
        T: windows::core::IntoParam<'a, IDWriteFontCollection>,
    {
        let format = unsafe {
            self.dwfactory.CreateTextFormat(
                PCWSTR(
                    formatdesc
                        .fontname
                        .encode_utf16()
                        .collect::<Vec<u16>>()
                        .as_ptr(),
                ),
                fontcollection,
                DWRITE_FONT_WEIGHT_NORMAL,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                formatdesc.size * 72.0 / 96.0,
                PCWSTR("ja-JP\0".encode_utf16().collect::<Vec<u16>>().as_ptr()),
            )?
        };
        let alignment = if formatdesc.align == 1 {
            DWRITE_TEXT_ALIGNMENT_CENTER
        } else if formatdesc.align == 2 {
            DWRITE_TEXT_ALIGNMENT_TRAILING
        } else {
            DWRITE_TEXT_ALIGNMENT_LEADING
        };
        unsafe { format.SetTextAlignment(alignment)? };
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
                .CreateSolidColorBrush(&color, &brushproperties)?
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
                .EndDraw(std::ptr::null_mut(), std::ptr::null_mut())?
        };
        Ok(())
    }
}
