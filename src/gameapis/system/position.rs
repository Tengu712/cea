use super::*;

pub fn system_position_sprite(manager: &mut EntityManager) {
    for (k, s, v) in manager.components.positions.iter() {
        if !s.is_active() {
            continue;
        }
        if let Some(n) = manager.components.sprites.get_mut(&k) {
            n.translation = v.clone();
        }
    }
}
