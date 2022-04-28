#[derive(Default, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Default, Debug)]
pub struct Rect {
    pub l: f32,
    pub r: f32,
    pub t: f32,
    pub b: f32,
    pub n: f32,
    pub f: f32,
}

#[derive(Default, Debug)]
pub struct Input {
    pub z: i16,
    pub x: i16,
    pub s: i16,
    pub left: i16,
    pub up: i16,
    pub right: i16,
    pub down: i16,
}

pub(super) type Velocity = Vector;
impl Velocity {
    pub(super) fn from_input(input: &Input) -> Self {
        let lr = (input.right > 0) as i32 - (input.left > 0) as i32;
        let ud = (input.up > 0) as i32 - (input.down > 0) as i32;
        let coef = if lr.abs() + ud.abs() == 2 {
            1.0 / std::f32::consts::SQRT_2
        } else {
            1.0
        };
        Self {
            x: lr as f32 * coef,
            y: ud as f32 * coef,
            z: 0.0,
        }
    }
}

pub(super) type Position = Vector;
impl Position {
    pub(super) fn update_with_velocity(self, velocity: &Velocity, speed: f32) -> Self {
        Self {
            x: self.x + velocity.x * speed,
            y: self.y + velocity.y * speed,
            z: self.z + velocity.z * speed,
        }
    }
    pub(super) fn restrict(self, rect: &Rect) -> Self {
        Self {
            x: self.x.max(rect.l).min(rect.r),
            y: self.y.max(rect.b).min(rect.t),
            z: self.z.max(rect.n).min(rect.f),
        }
    }
}
