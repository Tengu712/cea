use super::*;

pub fn system_fpsmeasure_text(manager: &mut EntityManager) {
    let end = std::time::Instant::now();
    for (k, v) in manager.components.fpsmeasures.iter_mut() {
        if let Some(n) = manager.components.texts.get_mut(k) {
            let since = end.duration_since(v.last);
            if since.as_secs() >= 1 {
                v.fps = (v.count as f32) / since.as_secs_f32();
                v.count = 0;
                v.last = end;
            }
            v.count += 1;
            n.text = format!("{:.1}fps", v.fps);
        }
    }
}

pub fn system_input_velocity_player(manager: &mut EntityManager) {
    for k in manager.components.playerinputs.keys() {
        if let Some(mut n) = manager.components.velocities.get_mut(k) {
            let lr = (manager.input.right > 0) as i32 - (manager.input.left > 0) as i32;
            let ud = (manager.input.up > 0) as i32 - (manager.input.down > 0) as i32;
            let coef = if lr.abs() + ud.abs() == 2 {
                1.0 / std::f32::consts::SQRT_2
            } else {
                1.0
            };
            n.direction.x = lr as f32 * coef;
            n.direction.y = ud as f32 * coef;
        }
    }
}

pub fn system_velocity_position(manager: &mut EntityManager) {
    for (k, v) in &manager.components.velocities {
        if let Some(mut n) = manager.components.positions.get_mut(&k) {
            n.x += v.direction.x * v.speed;
            n.y += v.direction.y * v.speed;
            n.z += v.direction.z * v.speed;
        }
    }
}

pub fn system_restrict_position(manager: &mut EntityManager) {
    for (k, v) in &manager.components.restricts {
        if let Some(mut n) = manager.components.positions.get_mut(&k) {
            n.x = n.x.max(v.l).min(v.r);
            n.y = n.y.max(v.b).min(v.t);
            n.z = n.z.max(v.n).min(v.f);
        }
    }
}

pub fn system_sameposition(manager: &mut EntityManager) {
    for (k, v) in &manager.components.samepositions {
        if let Some(id) = manager.entities.get(v.0) {
            let mut src_pos = Position::default();
            if let Some(pos) = manager.components.positions.get(id) {
                src_pos = pos.clone();
            }
            if let Some(dst_pos) = manager.components.positions.get_mut(&k) {
                *dst_pos = src_pos.clone();
            }
        }
    }
}

pub fn system_position_sprite(manager: &mut EntityManager) {
    for (k, v) in &manager.components.positions {
        if let Some(mut n) = manager.components.sprites.get_mut(&k) {
            n.translation = v.clone();
        }
    }
}

pub fn system_playeranimation_sprite(manager: &mut EntityManager) {
    for k in manager.components.playeranimations.keys() {
        if let Some(velocity) = manager.components.velocities.get(k) {
            if let Some(mut n) = manager.components.sprites.get_mut(k) {
                if velocity.direction.x > 0.0 {
                    n.imgid = Some(IMGID_FLAN_R0);
                } else if velocity.direction.x < 0.0 {
                    n.imgid = Some(IMGID_FLAN_L0);
                } else {
                    n.imgid = Some(IMGID_FLAN_B0);
                }
            }
        }
    }
}
