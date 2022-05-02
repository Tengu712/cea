use super::*;

pub fn create_delay_count(emngr: &mut EntityManager, count_max: i64) -> EntityID {
    let id = emngr.create_entity();
    emngr.coms.counters.insert(
        id,
        Counter {
            speed: 1,
            count: 0,
            count_max,
        },
    );
    id
}
