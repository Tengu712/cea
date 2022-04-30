use super::*;

pub fn system_update_counter(manager: &mut EntityManager) {
    for v in manager.components.counters.values_mut() {
        v.0 += 1;
    }
}
