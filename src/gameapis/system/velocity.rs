use super::*;

pub fn system_velocity_position(manager: &mut EntityManager) {
    for (k, s, v) in manager.components.velocities.iter() {
        if !s.is_active() {
            continue;
        }
        if let Some(n) = manager.components.positions.get_mut(&k) {
            n.x += v.direction.x * v.speed;
            n.y += v.direction.y * v.speed;
            n.z += v.direction.z * v.speed;
        }
    }
}
