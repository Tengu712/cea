use super::*;

pub use super::super::super::winapis::directwrite::text::TextAlign;

#[derive(Eq, PartialEq, Hash)]
pub enum TextFormat {
    Normal,
    Fps,
    Score,
    Option,
}

pub struct TextDesc {
    pub text: String,
    pub format: TextFormat,
    pub rect: [f32; 4],
    pub rgba: [f32; 4],
    pub align: TextAlign,
}
impl TextDesc {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            format: TextFormat::Normal,
            rect: [0.0, 1280.0, 0.0, 720.0],
            rgba: [1.0; 4],
            align: TextAlign::Left,
        }
    }
    pub fn set_text<T: std::string::ToString>(self, text: T) -> Self {
        let mut self_mut = self;
        self_mut.text = text.to_string();
        self_mut
    }
    pub fn set_format(self, format: TextFormat) -> Self {
        let mut self_mut = self;
        self_mut.format = format;
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
    pub fn set_align(self, align: TextAlign) -> Self {
        let mut self_mut = self;
        self_mut.align = align;
        self_mut
    }
}
impl PackingRequest for TextDesc {
    fn pack(self) -> Request {
        Request::DrawText(self)
    }
}
