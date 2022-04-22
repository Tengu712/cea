pub struct Enemy {
    pub hp: [u32; 2],
    pub pos_xy: [f32; 2],
}
impl Enemy {
    pub fn new() -> Self {
        Self {
            hp: [2000; 2],
            pos_xy: [0.0, 280.0],
        }
    }
    pub fn update(self, count_scene_f32: f32) -> Self {
        let pos_xy = [
            self.pos_xy[0],
            self.pos_xy[1] + (count_scene_f32 * 6.0).to_radians().cos() * 0.5,
        ];
        Self {
            hp: self.hp,
            pos_xy,
        }
    }
}
