use super::*;

pub use super::super::super::winapis::directwrite::text::TextAlign;

const TEXT_SIZE: usize = 128;

#[derive(Clone, Copy)]
pub struct TextDesc {
    pub text: [u8; TEXT_SIZE],
    pub rect: [f32; 4],
    pub rgba: [f32; 4],
    pub align: TextAlign,
}
impl TextDesc {
    pub fn new() -> Self {
        Self {
            text: [0; TEXT_SIZE],
            rect: [0.0, 1280.0, 0.0, 720.0],
            rgba: [1.0; 4],
            align: TextAlign::Left,
        }
    }
    pub fn set_text<T: ConvertStringArrayImpl>(self, text: T) -> Self {
        let mut self_mut = self;
        self_mut.text = text.as_text_array();
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

pub trait ConvertStringArrayImpl {
    fn as_text_array(self) -> [u8; TEXT_SIZE];
}
impl ConvertStringArrayImpl for String {
    fn as_text_array(self) -> [u8; TEXT_SIZE] {
        copy_text_array(self.as_bytes())
    }
}
impl ConvertStringArrayImpl for &str {
    fn as_text_array(self) -> [u8; TEXT_SIZE] {
        copy_text_array(self.as_bytes())
    }
}
fn copy_text_array(arr: &[u8]) -> [u8; TEXT_SIZE] {
    let mut res = [0; TEXT_SIZE];
    for i in 0..arr.len() {
        if i >= TEXT_SIZE {
            break;
        } else {
            res[i] = arr[i];
        }
    }
    res
}
