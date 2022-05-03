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
pub struct Text {
    pub layer: f32,
    pub text: String,
    pub rect: Rect,
    pub rgba: Vector4D,
    pub fontname: &'static str,
    pub size: f32,
    pub align: TextAlign,
}
impl Default for Text {
    fn default() -> Self {
        Self {
            layer: 0.0,
            text: String::default(),
            rect: Rect::default(),
            rgba: Vector4D::default(),
            fontname: "MS Gothic\0",
            size: 10.0,
            align: TextAlign::default(),
        }
    }
}
