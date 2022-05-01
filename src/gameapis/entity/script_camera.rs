use super::*;

pub fn create_script_camera(manager: &mut EntityManager) -> EntityID {
    let id = manager.create_entity();
    manager.insert_scripted_id(id, type_name::<MarkerCamera>());
    manager.components.counters.insert(
        id,
        Counter {
            speed: 1,
            count: 0,
            count_max: std::u64::MAX,
        },
    );
    id
}
