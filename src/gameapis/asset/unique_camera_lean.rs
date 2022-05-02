use super::*;

pub fn unique_camera_lean(emngr: &mut EntityManager) {
    if let Some(id) = emngr.unique_ids.get(UNIQUE_CAMERA_LEAN) {
        if let Some(n) = emngr.coms.counters.get_mut(id) {
            if emngr.input.s > 0 || (emngr.input.left > 0 && emngr.input.right > 0) {
            } else if emngr.input.left > 0 {
                n.count_max = (n.count_max - 5).max(-60).min(60);
            } else if emngr.input.right > 0 {
                n.count_max = (n.count_max + 5).max(-60).min(60);
            } else if n.count_max != 0 {
                n.count_max = n.count_max - n.count_max / n.count_max.abs() * 5;
            }
            emngr.camera.rot.y = -(n.count_max as f32 / 10.0).to_radians();
            emngr.camera.rot.z = (n.count_max as f32 / 20.0).to_radians();
        }
    }
}
