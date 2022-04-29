/// A component to change player's image.
pub mod playeranimation;
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

pub use playeranimation::*;
pub use playerinput::*;
pub use position::*;
pub use restrict::*;
pub use sameposition::*;
pub use sprite::*;
pub use text::*;
pub use velocity::*;

use super::asset::*;
use std::collections::HashMap;

pub trait SystemImpl<T, U> {
    fn process(update: &mut T, refer: &U);
}
pub struct System;

pub type EntityID = usize;
pub type EntityKey = &'static str;
pub type CContainer<T> = HashMap<EntityID, T>;
pub type Entities = HashMap<EntityKey, EntityID>;

#[derive(Default)]
pub struct Components {
    pub next_entity_id: usize,
    pub entities: Entities,
    pub input: Input,
    pub playeranimations: CContainer<PlayerAnimation>,
    pub playerinputs: CContainer<PlayerInput>,
    pub positions: CContainer<Position>,
    pub restricts: CContainer<RestrictRect>,
    pub samepositions: CContainer<SamePosition>,
    pub sprites: CContainer<Sprite>,
    pub texts: CContainer<Text>,
    pub velocities: CContainer<Velocity>,
}
impl Components {
    pub fn update(&mut self) {
        System::process(&mut self.velocities, &(&self.playerinputs, &self.input));
        System::process(&mut self.positions, &self.velocities);
        System::process(&mut self.positions, &self.restricts);
        System::process(&mut self.samepositions, &(&self.positions, &self.entities));
        System::process(&mut self.positions, &self.samepositions);
        System::process(
            &mut self.sprites,
            &(&self.playeranimations, &self.velocities),
        );
        System::process(&mut self.sprites, &self.positions);
    }
}

#[derive(Default)]
pub struct Input {
    pub z: i16,
    pub x: i16,
    pub s: i16,
    pub left: i16,
    pub up: i16,
    pub right: i16,
    pub down: i16,
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
