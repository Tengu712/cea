use super::*;

/// If source entity is removed, the entity will be removed.
pub fn system_same_position_2d(emngr: &mut EntityManager) {
    let mut remove_ids = Vec::new();
    for (k, s, v) in emngr.coms.sameposition2ds.iter() {
        if !s.is_active() {
            continue;
        }
        let src_pos = if let Some(n) = emngr.coms.positions.get(&v.0) {
            n.clone()
        } else {
            remove_ids.push(*k);
            continue;
        };
        if let Some(dst_pos) = emngr.coms.positions.get_mut(k) {
            dst_pos.x = src_pos.x;
            dst_pos.y = src_pos.y;
        } else {
            // Error : Position must be set at the same time.
            remove_ids.push(*k);
        }
    }
    for i in remove_ids {
        emngr.remove_entity(&i);
    }
}
