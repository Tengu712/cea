use super::*;

pub struct MarkerCamera1;

pub fn script_camera_1(manager: &mut EntityManager) {
    let count = match manager.scripted_ids.get(type_name::<MarkerCamera1>()) {
        Some(ids) => match ids.iter().next() {
            Some(id) => match manager.components.counters.get(id) {
                Some(n) => n.count,
                None => return,
            },
            None => return,
        },
        None => return,
    };
    manager.camera.pos.y = (count % 350) as f32;
}
