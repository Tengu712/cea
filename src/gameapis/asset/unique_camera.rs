use super::*;

pub fn unique_camera(emngr: &mut EntityManager) {
    if let Some(id) = emngr.unique_ids.get(UNIQUE_CAMERA) {
        if let Some(n) = emngr.coms.counters.get(id) {
            emngr.camera.pos.y = (n.count % 350) as f32;
        }
    }
}
