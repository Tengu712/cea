use super::*;

pub struct MarkerTitleText;

/// Do title text's animation with counter.
pub fn script_title_text(manager: &mut EntityManager) {
    if let Some(ids) = manager.scripted_ids.get(type_name::<MarkerTitleText>()) {
        for id in ids {
            if let Some(counter) = manager.components.counters.get(id) {
                if let Some(n) = manager.components.texts.get_mut(id) {
                    n.rgba.w = (counter.count as f32).to_radians().cos().abs();
                }
            }
        }
    }
}
