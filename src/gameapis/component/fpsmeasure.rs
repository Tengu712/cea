use super::*;

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
impl SystemImpl<(FpsMeasure, EntityID), ()> for System {
    fn process(update: &mut (FpsMeasure, EntityID), _: &()) {
        let (measure, _) = update;
        let end = Instant::now();
        let since = end.duration_since(measure.last);
        if since.as_secs() >= 1 {
            measure.fps = (measure.count as f32) / since.as_secs_f32();
            measure.count = 0;
            measure.last = end;
        }
        measure.count += 1;
    }
}
impl SystemImpl<CContainer<Text>, (FpsMeasure, EntityID)> for System {
    fn process(update: &mut CContainer<Text>, refer: &(FpsMeasure, EntityID)) {
        let (measure, id) = refer;
        if let Some(mut n) = update.get_mut(id) {
            n.text = format!("{:.1}fps", measure.fps);
        }
    }
}
