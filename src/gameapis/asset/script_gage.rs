use super::*;

pub struct MarkerGage;

pub fn script_gage(manager: &mut EntityManager) {
    if let Some(ids) = manager.scripted_ids.get(type_name::<MarkerGage>()) {
        for id in ids {
            if let Some(sprite) = manager.components.sprites.get_mut(id) {
                if let Some(counter) = manager.components.counters.get(id) {
                    let deg = 360.0 * (counter.count as f32).max(0.0)
                        / (counter.count_max as f32).max(1.0);
                    sprite.rotation.z = deg.to_radians();
                }
            }
        }
    }
}
