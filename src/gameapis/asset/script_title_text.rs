use super::*;

pub struct MarkerTitleText;

/// Do title text's animation with counter.
pub fn script_title_text(emngr: &mut EntityManager) {
    if let Some(ids) = emngr.scripted_ids.get(type_name::<MarkerTitleText>()) {
        for id in ids {
            if let Some(counter) = emngr.coms.counters.get(id) {
                if let Some(n) = emngr.coms.texts.get_mut(id) {
                    n.rgba.w = (counter.count as f32).to_radians().cos().abs();
                }
            }
        }
    }
}
