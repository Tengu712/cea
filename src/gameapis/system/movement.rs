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

pub fn system_restrict_position(manager: &mut EntityManager) {
    for (k, s, v) in manager.components.restricts.iter() {
        if !s.is_active() {
            continue;
        }
        if let Some(n) = manager.components.positions.get_mut(&k) {
            n.x = n.x.max(v.l).min(v.r);
            n.y = n.y.max(v.b).min(v.t);
            n.z = n.z.max(v.n).min(v.f);
        }
    }
}

pub fn system_same_position_2d(manager: &mut EntityManager) {
    for (k, s, v) in manager.components.sameposition2ds.iter() {
        if !s.is_active() {
            continue;
        }
        let src_pos = match manager.components.positions.get(&v.0) {
            Some(n) => n.clone(),
            None => continue,
        };
        if let Some(dst_pos) = manager.components.positions.get_mut(k) {
            dst_pos.x = src_pos.x;
            dst_pos.y = src_pos.y;
        }
    }
}

pub fn system_position_sprite(manager: &mut EntityManager) {
    for (k, s, v) in manager.components.positions.iter() {
        if !s.is_active() {
            continue;
        }
        if let Some(n) = manager.components.sprites.get_mut(&k) {
            n.translation = v.clone();
        }
    }
}
