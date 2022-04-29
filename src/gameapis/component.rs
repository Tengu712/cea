/// A component to measure fps.
pub mod fpsmeasure;
/// A component to change player's image.
pub mod playeranimation;
/// A component to cange player slow's image.
pub mod playerslowanimation;
/// A component to change velocity based on input especially for player.
pub mod playerinput;
/// A component to change translation of Sprite.
pub mod position;
/// A component to restrict position.
pub mod restrict;
/// A component to change position same as player's position.
pub mod sameposition;
/// A component to draw sprite on screen.
pub mod sprite;
/// A component to draw text on screen.
pub mod text;
/// A component to change Position.
pub mod velocity;

pub use fpsmeasure::*;
pub use playeranimation::*;
pub use playerslowanimation::*;
pub use playerinput::*;
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
