use super::*;

pub fn unique_camera(manager: &mut EntityManager) {
    if let Some(id) = manager.unique_ids.get(UNIQUE_CAMERA) {
        if let Some(n) = manager.components.counters.get(id) {
            manager.camera.pos.y = (n.count % 350) as f32;
        }
    }
}
