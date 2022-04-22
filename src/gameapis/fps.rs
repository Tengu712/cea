use std::time::Instant;

pub struct FpsData {
    fps: f32,
    count: u32,
    last: Instant,
}
impl FpsData {
    pub fn new() -> Self {
        Self {
            fps: 0.0,
            count: 0,
            last: Instant::now(),
        }
    }
    pub fn update(self) -> Self {
        let end = Instant::now();
        let since = end.duration_since(self.last);
        if since.as_secs() >= 1 {
            Self {
                fps: (self.count as f32) / since.as_secs_f32(),
                count: 1,
                last: end,
            }
        } else {
            Self {
                fps: self.fps,
                count: self.count + 1,
                last: self.last,
            }
        }
    }
    pub fn get_fps(&self) -> f32 {
        self.fps
    }
}
