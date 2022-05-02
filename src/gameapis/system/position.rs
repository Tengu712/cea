use super::*;

pub fn system_position_sprite(emngr: &mut EntityManager) {
    for (k, s, v) in emngr.coms.positions.iter() {
        if !s.is_active() {
            continue;
        }
        if let Some(n) = emngr.coms.sprites.get_mut(&k) {
            n.translation = v.clone();
        }
    }
}
