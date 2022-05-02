use super::*;

/// Update fps measure and change text.
pub fn system_fpsmeasure(emngr: &mut EntityManager) {
    let end = std::time::Instant::now();
    for (k, s, v) in emngr.coms.fpsmeasures.iter_mut() {
        if !s.is_active() {
            continue;
        }
        if let Some(n) = emngr.coms.texts.get_mut(k) {
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
