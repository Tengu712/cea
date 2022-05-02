use super::*;

pub fn system_update_counter(manager: &mut EntityManager) {
    for (_, s, v) in manager.components.counters.iter_mut() {
        if !s.is_active() {
            continue;
        }
        v.count = (v.count + v.speed).min(v.count_max);
    }
}
