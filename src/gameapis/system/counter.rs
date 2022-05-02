use super::*;

pub fn system_update_counter(emngr: &mut EntityManager) {
    for (_, s, v) in emngr.coms.counters.iter_mut() {
        if !s.is_active() {
            continue;
        }
        v.count = (v.count + v.speed).min(v.count_max);
    }
}
