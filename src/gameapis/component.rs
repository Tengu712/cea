pub mod fpsmeasure;
pub mod player;
pub mod position;
pub mod restrict;
pub mod sameposition;
pub mod sprite;
pub mod text;
pub mod velocity;

pub use fpsmeasure::*;
pub use player::*;
pub use position::*;
pub use restrict::*;
pub use sameposition::*;
pub use sprite::*;
pub use text::*;
pub use velocity::*;

use super::*;

#[derive(Default)]
pub struct Components {
    pub fpsmeasures: CContainer<FpsMeasure>,
    pub playeranimations: CContainer<PlayerAnimation>,
    pub playerslowanimations: CContainer<PlayerSlowAnimation>,
    pub playerinputs: CContainer<PlayerInput>,
    pub positions: CContainer<Position>,
    pub restricts: CContainer<RestrictRect>,
    pub samepositions: CContainer<SamePosition>,
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
