use super::*;

pub enum TextAlign2 {
    Left,
    Center,
    Right,
}
pub struct Text {
    pub layer: u32,
    pub text: String,
    pub rect: Rect,
    pub rgba: Vector4D,
    pub fontname: &'static str,
    pub size: f32,
    pub align: TextAlign2,
}