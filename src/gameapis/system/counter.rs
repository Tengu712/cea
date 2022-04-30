use super::*;

pub fn system_update_counter(manager: &mut EntityManager) {
    for v in manager.components.counters.values_mut() {
        v.count = (v.count + 1).min(v.count_max);
    }
}
