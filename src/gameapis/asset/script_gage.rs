use super::*;

pub struct MarkerGage;

pub fn script_gage(emngr: &mut EntityManager) {
    if let Some(ids) = emngr.scripted_ids.get(type_name::<MarkerGage>()) {
        for id in ids {
            if let Some(sprite) = emngr.coms.sprites.get_mut(id) {
                if let Some(counter) = emngr.coms.counters.get(id) {
                    let deg = 360.0 * (counter.count as f32).max(0.0)
                        / (counter.count_max as f32).max(1.0);
                    sprite.rotation.z = deg.to_radians();
                }
            }
        }
    }
}
