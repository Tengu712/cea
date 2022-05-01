use super::*;

pub fn system_update_counter(manager: &mut EntityManager) {
    for (_, s, v) in manager.components.counters.iter_mut() {
        if !s.is_active() {
            continue;
        }
        v.count = (v.count + v.speed).min(v.count_max);
    }
}
/// Reflect counter value to text.
pub fn system_value_text(manager: &mut EntityManager) {
    for (k, s, v) in manager.components.valuetexts.iter() {
        if !s.is_active() {
            continue;
        }
        if let Some(counter) = manager.components.counters.get(k) {
            if let Some(n) = manager.components.texts.get_mut(k) {
                match v.format {
                    Some(f) => n.text = f(counter.count),
                    None => n.text = counter.count.to_string(),
                }
            }
        }
    }
}
/// Update fps measure and change text.
pub fn system_fpsmeasure(manager: &mut EntityManager) {
    let end = std::time::Instant::now();
    for (k, s, v) in manager.components.fpsmeasures.iter_mut() {
        if !s.is_active() {
            continue;
        }
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
