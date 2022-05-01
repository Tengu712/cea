use super::*;

pub fn system_update_counter(manager: &mut EntityManager) {
    for v in manager.components.counters.values_mut() {
        v.count = (v.count + 1).min(v.count_max);
    }
}
/// Update fps measure and change text.
pub fn system_fpsmeasure(manager: &mut EntityManager) {
    let end = std::time::Instant::now();
    for (k, v) in manager.components.fpsmeasures.iter_mut() {
        if let Some(n) = manager.components.texts.get_mut(k) {
            let since = end.duration_since(v.last);
            if since.as_secs() >= 1 {
                v.fps = (v.count as f32) / since.as_secs_f32();
                v.count = 0;
                v.last = end;
            }
            v.count += 1;
            n.text = format!("{:.1}fps", v.fps);
        }
    }
}
