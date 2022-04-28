use std::collections::HashMap;

#[derive(Default)]
pub struct Components {
    pub next_entity_id: usize,
    pub input: Input,
    pub velocities: HashMap<usize, Velocity>,
    pub positions: HashMap<usize, Position>,
    pub sprites: HashMap<usize, Sprite>,
}
pub trait ComponentUpdaterImpl<T, U> {
    fn process(update: &mut HashMap<usize, T>, refer: &HashMap<usize, U>);
}
pub struct ComponentUpdater;

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
    pub n: f32,
    pub f: f32,
}

/// A component to change Position.
#[derive(Default)]
pub struct Velocity {
    pub direction: Vector,
    pub speed: f32,
}

/// A component to change Sprite.translation.
pub type Position = Vector;

/// A component to draw sprite on screen.
#[derive(Default)]
pub struct Sprite {
    pub imgid: Option<&'static str>,
    pub layer: u32,
    pub translation: Vector,
    pub rotation: Vector,
    pub scaling: Vector,
    pub color: Vector4D,
}

impl ComponentUpdaterImpl<Position, Velocity> for ComponentUpdater {
    fn process(update: &mut HashMap<usize, Position>, refer: &HashMap<usize, Velocity>) {
        for (k, v) in update {
            if let Some(n) = refer.get(k) {
                v.x += n.direction.x * n.speed;
                v.y += n.direction.y * n.speed;
                v.z += n.direction.z * n.speed;
            }
        }
    }
}
impl ComponentUpdaterImpl<Sprite, Position> for ComponentUpdater {
    fn process(update: &mut HashMap<usize, Sprite>, refer: &HashMap<usize, Position>) {
        for (k, v) in update {
            if let Some(n) = refer.get(k) {
                v.translation = n.clone();
            }
        }
    }
}
