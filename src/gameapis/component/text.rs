use super::*;

pub enum TextAlign {
    Left,
    Center,
    Right,
}
impl Default for TextAlign {
    fn default() -> Self {
        TextAlign::Left
    }
}
#[derive(Default)]
pub struct Text {
    pub text: String,
    pub rect: Rect,
    pub rgba: Vector4D,
    pub fontname: &'static str,
    pub size: f32,
    pub align: TextAlign,
}
