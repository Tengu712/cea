use super::*;

pub fn create_camera_lean(manager: &mut EntityManager) -> EntityID {
    let id = manager.create_entity();
    manager.components.counters.insert(
        id,
        Counter {
            speed: 0,
            count: 65,
            count_max: 65,
        },
    );
    id
}
