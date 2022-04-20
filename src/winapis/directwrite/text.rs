use super::*;

pub enum TextAlign {
    Left,
    Center,
    Right,
}

/// Use it when drawing text.
pub struct TextDesc {
    pub text: String,
    pub rect: [f32; 4],
    pub rgba: [f32; 4],
    pub align: TextAlign,
}
pub trait TextDescImpl<T> {
    fn set_text(self, text: T) -> Self;
}
impl TextDescImpl<String> for TextDesc {
    fn set_text(self, text: String) -> Self {
        let mut self_mut = self;
        self_mut.text = text;
        self_mut
    }
}
impl TextDescImpl<&str> for TextDesc {
    fn set_text(self, text: &str) -> Self {
        let mut self_mut = self;
        self_mut.text = String::from(text);
        self_mut
    }
}
impl TextDesc {
    pub fn new() -> Self {
        Self {
            text: String::default(),
            rect: [0.0, 1280.0, 0.0, 720.0],
            rgba: [1.0; 4],
            align: TextAlign::Left,
        }
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
    pub fn set_align(self, align: TextAlign) -> Self {
        let mut self_mut = self;
        self_mut.align = align;
        self_mut
    }
}

impl DWriteApp {
    /// Draw text. To call it, user give it DrawTextDesc struct.
    pub fn draw_text(
        &self,
        desc: &TextDesc,
        format: &IDWriteTextFormat,
    ) -> Result<(), WErr> {
        let alignment = match desc.align {
            TextAlign::Left => DWRITE_TEXT_ALIGNMENT_LEADING,
            TextAlign::Center => DWRITE_TEXT_ALIGNMENT_CENTER,
            TextAlign::Right => DWRITE_TEXT_ALIGNMENT_TRAILING,
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
    /// Create text format.
    pub fn create_text_format(&self, font: &str, size: f32) -> Result<IDWriteTextFormat, WErr> {
        unsafe {
            self.dwfactory
                .CreateTextFormat(
                    PCWSTR(font.encode_utf16().collect::<Vec<u16>>().as_ptr()),
                    None,
                    DWRITE_FONT_WEIGHT_NORMAL,
                    DWRITE_FONT_STYLE_NORMAL,
                    DWRITE_FONT_STRETCH_NORMAL,
                    size * 72.0 / 96.0,
                    PCWSTR("ja-JP\0".encode_utf16().collect::<Vec<u16>>().as_ptr()),
                )
                .map_err(|_| raise_err(EKnd::Runtime, "Creation text format failed"))
        }
    }
}
