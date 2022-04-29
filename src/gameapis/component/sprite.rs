use super::*;

#[derive(Default)]
pub struct Sprite {
    pub layer: u32,
    pub imgid: Option<&'static str>,
    pub translation: Vector,
    pub rotation: Vector,
    pub scaling: Vector,
    pub color: Vector4D,
}
