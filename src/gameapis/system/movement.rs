use super::*;

pub fn system_velocity_position(manager: &mut EntityManager) {
    for (k, v) in &manager.components.velocities {
        if let Some(n) = manager.components.positions.get_mut(&k) {
            n.x += v.direction.x * v.speed;
            n.y += v.direction.y * v.speed;
            n.z += v.direction.z * v.speed;
        }
    }
}

pub fn system_restrict_position(manager: &mut EntityManager) {
    for (k, v) in &manager.components.restricts {
        if let Some(n) = manager.components.positions.get_mut(&k) {
            n.x = n.x.max(v.l).min(v.r);
            n.y = n.y.max(v.b).min(v.t);
            n.z = n.z.max(v.n).min(v.f);
        }
    }
}

pub fn system_position_sprite(manager: &mut EntityManager) {
    for (k, v) in &manager.components.positions {
        if let Some(n) = manager.components.sprites.get_mut(&k) {
            n.translation = v.clone();
        }
    }
}
