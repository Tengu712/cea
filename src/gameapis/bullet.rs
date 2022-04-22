pub struct Bullet {
    pub vel: f32,
    pub deg: f32,
    pub pos: [f32; 2],
}
impl Bullet {
    pub fn new() -> Self {
        Self {
            vel: 0.0,
            deg: 0.0,
            pos: [0.0; 2],
        }
    }
    pub fn set_vel(self, vel: f32) -> Self {
        let mut self_mut = self;
        self_mut.vel = vel;
        self_mut
    }
    pub fn set_deg(self, deg: f32) -> Self {
        let mut self_mut = self;
        self_mut.deg = deg;
        self_mut
    }
    pub fn update(self, rect: [f32; 4]) -> Option<Self> {
        let pos = [
            self.pos[0] + self.vel * self.deg.to_radians().cos(),
            self.pos[1] + self.vel * self.deg.to_radians().sin(),
        ];
        if self.pos[0] < rect[0]
            || self.pos[0] > rect[1]
            || self.pos[1] > rect[2]
            || self.pos[1] < rect[3]
        {
            None
        } else {
            Some(Self {
                vel: self.vel,
                deg: self.deg,
                pos,
            })
        }
    }
}
