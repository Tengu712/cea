use super::*;

#[derive(Clone, Copy)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}
const TEXT_SIZE: usize = 128;
#[derive(Clone, Copy)]
pub struct TextDesc {
    pub text: [u8; TEXT_SIZE],
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
