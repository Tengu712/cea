use super::*;

pub fn create_script_1_1(manager: &mut EntityManager) -> EntityID {
    let id = manager.create_entity();
    manager.insert_scripted_id(id, type_name::<Marker11>());
    manager.components.counters.insert(
        id,
        Counter {
            speed: 1,
            count: 0,
            count_max: 3600,
        },
    );
    id
}
