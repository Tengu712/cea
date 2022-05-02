use super::*;

/// Launch player's bullet with counter and input.
pub fn unique_player_shot(manager: &mut EntityManager) {
    let id = if let Some(n) = manager.unique_ids.get(UNIQUE_PLAYER) {
        *n
    } else {
        return;
    };
    let mut v = Vec::new();
    if let Some(counter) = manager.components.counters.get(&id) {
        if counter.count % 6 != 0 || manager.input.z == 0 {
            return;
        }
        if let Some(pos) = manager.components.positions.get(&id) {
            v.push(pos.clone());
        }
    }
    for i in v {
        let _ = create_player_bullet(manager, i.x - 20.0, i.y + 50.0);
        let _ = create_player_bullet(manager, i.x + 20.0, i.y + 50.0);
    }
}
