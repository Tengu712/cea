use super::*;

#[derive(Default)]
pub struct Components {
    pub counters: CContainer<Counter>,
    pub fpsmeasures: CContainer<FpsMeasure>,
    pub positions: CContainer<Position>,
    pub restricts: CContainer<RestrictRect>,
    pub sprites: CContainer<Sprite>,
    pub texts: CContainer<Text>,
    pub velocities: CContainer<Velocity>,
}

#[derive(Default, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[derive(Default, Clone)]
pub struct Vector4D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[derive(Default)]
pub struct Rect {
    pub l: f32,
    pub r: f32,
    pub t: f32,
    pub b: f32,
}
#[derive(Default)]
pub struct Rect3D {
    pub l: f32,
    pub r: f32,
    pub b: f32,
    pub t: f32,
    pub n: f32,
    pub f: f32,
}

// counter
#[derive(Default)]
pub struct Counter {
    pub count: u32,
    pub count_max: u32,
}
pub struct FpsMeasure {
    pub fps: f32,
    pub count: u32,
    pub last: std::time::Instant,
}
// fpsmeasure
impl Default for FpsMeasure {
    fn default() -> Self {
        Self {
            fps: 0.0,
            count: 0,
            last: std::time::Instant::now(),
        }
    }
}
// movement
pub type Position = Vector;
pub type RestrictRect = Rect3D;
#[derive(Default)]
pub struct Velocity {
    pub direction: Vector,
    pub speed: f32,
}
// graphic
#[derive(Default)]
pub struct Sprite {
    pub visible: bool,
    pub imgid: Option<&'static str>,
    pub translation: Vector,
    pub rotation: Vector,
    pub scaling: Vector,
    pub color: Vector4D,
}
pub enum TextAlign {
    Left,
    Center,
    Right,
}
pub struct Text {
    pub visible: bool,
    pub text: String,
    pub rect: Rect,
    pub rgba: Vector4D,
    pub fontname: &'static str,
    pub size: f32,
    pub align: TextAlign,
}
