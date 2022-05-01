use super::*;

pub fn create_script_camera_lean(manager: &mut EntityManager) -> EntityID {
    let id = manager.create_entity();
    manager.insert_scripted_id(id, type_name::<MarkerCameraLean>());
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
