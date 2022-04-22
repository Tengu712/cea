use super::input::KeyStates;

const P_SPD: f32 = 8.0;

pub struct Player {
    pub pos: [f32; 2],
}
impl Player {
    pub fn new() -> Self {
        Self { pos: [0.0, -280.0] }
    }
    pub fn update(self, rect: [f32; 4], keystates: &KeyStates) -> Self {
        let inp_x = (keystates.right > 0) as i32 - (keystates.left > 0) as i32;
        let inp_y = (keystates.up > 0) as i32 - (keystates.down > 0) as i32;
        let c_spd = if keystates.s > 0 { 0.5 } else { 1.0 }
            / if inp_x.abs() + inp_y.abs() == 2 {
                std::f32::consts::SQRT_2
            } else {
                1.0
            };
        let pos = [
            self.pos[0] + inp_x as f32 * P_SPD * c_spd,
            self.pos[1] + inp_y as f32 * P_SPD * c_spd,
        ];
        let pos = [
            pos[0].max(rect[0]).min(rect[1]),
            pos[1].max(rect[3]).min(rect[2]),
        ];
        Self { pos }
    }
}
