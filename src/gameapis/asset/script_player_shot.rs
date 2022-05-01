use super::*;

pub struct MarkerPlayerShot;

/// Launch player's bullet with counter and input.
pub fn script_player_shot(manager: &mut EntityManager) {
    let mut v = Vec::new();
    if let Some(ids) = manager.scripted_ids.get(type_name::<MarkerPlayerShot>()) {
        for id in ids {
            if let Some(counter) = manager.components.counters.get(id) {
                if counter.count % 6 != 0 || manager.input.z == 0 {
                    continue;
                }
                if let Some(pos) = manager.components.positions.get(id) {
                    v.push(pos.clone());
                }
            }
        }
    }
    for i in v {
        let _ = create_player_bullet(manager, i.x - 20.0, i.y + 50.0);
        let _ = create_player_bullet(manager, i.x + 20.0, i.y + 50.0);
    }
}
