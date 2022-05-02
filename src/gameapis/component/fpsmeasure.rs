pub struct FpsMeasure {
    pub fps: f32,
    pub count: u32,
    pub last: std::time::Instant,
}
impl Default for FpsMeasure {
    fn default() -> Self {
        Self {
            fps: 0.0,
            count: 0,
            last: std::time::Instant::now(),
        }
    }
}
