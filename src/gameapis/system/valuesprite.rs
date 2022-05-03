use super::*;

/// Reflect counter value to text.
pub fn system_value_sprite(emngr: &mut EntityManager) {
    for (k, s, v) in emngr.coms.valuesprites.iter() {
        if !s.is_active() {
            continue;
        }
        if let Some(counter) = emngr.coms.counters.get(k) {
            if let Some(n) = emngr.coms.sprites.get_mut(k) {
                if let Some(f) = v.0 {
                    *n = f(counter)
                }
            }
        }
    }
}
