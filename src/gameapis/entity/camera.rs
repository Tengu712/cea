use super::*;

pub fn create_camera(emngr: &mut EntityManager) -> EntityID {
    let id = emngr.create_entity();
    emngr.coms.counters.insert(
        id,
        Counter {
            speed: 1,
            count: 0,
            count_max: std::i64::MAX,
        },
    );
    id
}
