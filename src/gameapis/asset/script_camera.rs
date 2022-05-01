use super::*;

pub struct MarkerCamera;

pub fn script_camera(manager: &mut EntityManager) {
    if let Some(ids) = manager.scripted_ids.get(type_name::<MarkerCamera>()) {
        for id in ids {
            if let Some(n) = manager.components.counters.get(id) {
                manager.camera.pos.y = (n.count % 350) as f32;
            }
        }
    }
}
