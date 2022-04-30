use super::*;

pub const TITLE_TEXT_KEY: EntityKey = "TitleText";

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
/// Do title text's animation with counter.
pub fn system_title_text_animation(manager: &mut EntityManager) {
    if let Some(id) = manager.entities.get(TITLE_TEXT_KEY) {
        if let Some(counter) = manager.components.counters.get(id) {
            if let Some(n) = manager.components.texts.get_mut(id) {
                n.rgba.w = (counter.count as f32).to_radians().cos().abs();
            }
        }
    }
}
