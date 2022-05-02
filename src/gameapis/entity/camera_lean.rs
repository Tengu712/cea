use super::*;

pub fn create_camera_lean(emngr: &mut EntityManager) -> EntityID {
    let id = emngr.create_entity();
    emngr.coms.counters.insert(
        id,
        Counter {
            speed: 0,
            count: 65,
            count_max: 65,
        },
    );
    id
}
