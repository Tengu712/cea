use std::time::Instant;

pub struct FpsMeasure {
    pub fps: f32,
    pub count: u32,
    pub last: Instant,
}
impl Default for FpsMeasure {
    fn default() -> Self {
        Self {
            fps: 0.0,
            count: 0,
            last: Instant::now(),
        }
    }
}
