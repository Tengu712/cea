use super::*;

pub fn system_same_position_2d(manager: &mut EntityManager) {
    for (k, s, v) in manager.components.sameposition2ds.iter() {
        if !s.is_active() {
            continue;
        }
        let src_pos = match manager.components.positions.get(&v.0) {
            Some(n) => n.clone(),
            None => continue,
        };
        if let Some(dst_pos) = manager.components.positions.get_mut(k) {
            dst_pos.x = src_pos.x;
            dst_pos.y = src_pos.y;
        }
    }
}
