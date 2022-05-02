use super::*;

pub fn unique_camera_lean(manager: &mut EntityManager) {
    if let Some(id) = manager.unique_ids.get(UNIQUE_CAMERA_LEAN) {
        if let Some(n) = manager.components.counters.get_mut(id) {
            if manager.input.s > 0 || (manager.input.left > 0 && manager.input.right > 0) {
            } else if manager.input.left > 0 {
                n.count_max = (n.count_max - 5).max(-60).min(60);
            } else if manager.input.right > 0 {
                n.count_max = (n.count_max + 5).max(-60).min(60);
            } else if n.count_max != 0 {
                n.count_max = n.count_max - n.count_max / n.count_max.abs() * 5;
            }
            manager.camera.rot.y = -(n.count_max as f32 / 10.0).to_radians();
            manager.camera.rot.z = (n.count_max as f32 / 20.0).to_radians();
        }
    }
}
