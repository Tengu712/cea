use std::collections::HashMap;

pub trait SystemImpl<T, U> {
    fn process(update: &mut T, refer: &U);
}
pub struct System;

#[derive(Default)]
pub struct Components {
    pub next_entity_id: usize,
    pub input: Input,
    pub playerinputs: HashMap<usize, PlayerInput>,
    pub velocities: HashMap<usize, Velocity>,
    pub positions: HashMap<usize, Position>,
    pub sprites: HashMap<usize, Sprite>,
}
impl Components {
    pub fn update(&mut self) {
        System::process(&mut self.velocities, &(&self.playerinputs, &self.input));
        System::process(&mut self.positions, &self.velocities);
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
    pub n: f32,
    pub f: f32,
}

/// A component to change Position.
#[derive(Default)]
pub struct Velocity {
    pub direction: Vector,
    pub speed: f32,
}

/// A component to change translation of Sprite.
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

/// A component to change velocity based on input especially for player.
pub struct PlayerInput;

impl SystemImpl<HashMap<usize, Position>, HashMap<usize, Velocity>> for System {
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
impl SystemImpl<HashMap<usize, Sprite>, HashMap<usize, Position>> for System {
    fn process(update: &mut HashMap<usize, Sprite>, refer: &HashMap<usize, Position>) {
        for (k, v) in update {
            if let Some(n) = refer.get(k) {
                v.translation = n.clone();
            }
        }
    }
}
impl SystemImpl<HashMap<usize, Velocity>, (&HashMap<usize, PlayerInput>, &Input)> for System {
    fn process(
        update: &mut HashMap<usize, Velocity>,
        refer: &(&HashMap<usize, PlayerInput>, &Input),
    ) {
        let (pi_map, input) = refer;
        for (k, _) in pi_map.into_iter() {
            if let Some(mut n) = update.get_mut(k) {
                let lr = (input.right > 0) as i32 - (input.left > 0) as i32;
                let ud = (input.up > 0) as i32 - (input.down > 0) as i32;
                let coef = if lr.abs() + ud.abs() == 2 {
                    1.0 / std::f32::consts::SQRT_2
                } else {
                    1.0
                };
                n.direction.x = lr as f32 * coef;
                n.direction.y = ud as f32 * coef;
            }
        }
    }
}
