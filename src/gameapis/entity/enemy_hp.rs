use super::*;

pub fn create_enemy_hp(manager: &mut EntityManager, hp_max: i64) -> EntityID {
    let id = manager.create_entity();
    manager.components.counters.insert(
        id,
        Counter {
            speed: 0,
            count: 0,
            count_max: hp_max,
        },
    );
    id
}
