use super::*;

pub fn create_delay_count(manager: &mut EntityManager) -> EntityID {
    let id = manager.create_entity();
    manager.components.counters.insert(
        id,
        Counter {
            speed: 1,
            count: 0,
            count_max: 10,
        },
    );
    id
}
