use super::*;

pub fn create_camera(manager: &mut EntityManager) -> EntityID {
    let id = manager.create_entity();
    manager.components.counters.insert(
        id,
        Counter {
            speed: 1,
            count: 0,
            count_max: std::i64::MAX,
        },
    );
    id
}
