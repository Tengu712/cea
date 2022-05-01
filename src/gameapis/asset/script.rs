use super::*;

pub struct MarkerPlayer;
pub struct MarkerPlayerSlow;
pub struct MarkerPlayerShot;
pub struct MarkerTitleText;

/// Change player's velocity with input. Then, it does player's animation with input.
pub fn script_player(manager: &mut EntityManager) {
    if let Some(ids) = manager.scripted_ids.get(type_name::<MarkerPlayer>()) {
        for id in ids {
            if let Some(n) = manager.components.velocities.get_mut(id) {
                let lr = (manager.input.right > 0) as i32 - (manager.input.left > 0) as i32;
                let ud = (manager.input.up > 0) as i32 - (manager.input.down > 0) as i32;
                let coef = if lr.abs() + ud.abs() == 2 {
                    1.0 / std::f32::consts::SQRT_2
                } else {
                    1.0
                };
                n.direction.x = lr as f32 * coef;
                n.direction.y = ud as f32 * coef;
                if let Some(n) = manager.components.sprites.get_mut(id) {
                    if lr > 0 {
                        n.imgid = Some(IMGID_FLAN_R0);
                    } else if lr < 0 {
                        n.imgid = Some(IMGID_FLAN_L0);
                    } else {
                        n.imgid = Some(IMGID_FLAN_B0);
                    }
                }
            }
        }
    }
}
/// Launch player's bullet with counter and input.
pub fn script_player_shot(manager: &mut EntityManager) {
    if let Some(ids) = manager.scripted_ids.get(type_name::<MarkerPlayerShot>()) {
        for id in ids {
            if let Some(counter) = manager.components.counters.get(id) {
                if counter.count % 60 == 1 && manager.input.z > 0 {
                    println!("shoot!");
                }
            }
        }
    }
}
/// Change player slow circle's animation with input.
pub fn script_player_slow(manager: &mut EntityManager) {
    if let Some(ids) = manager.scripted_ids.get(type_name::<MarkerPlayerSlow>()) {
        for id in ids {
            const SLOWCIRCLE_SIZE: f32 = 140.0;
            if let Some(n) = manager.components.sprites.get_mut(id) {
                let cnt = manager.input.s;
                n.visible = cnt > 0;
                let abs = n.rotation.z.abs();
                let sign = if abs == 0.0 {
                    1.0
                } else {
                    n.rotation.z / abs as f32
                };
                if cnt > 0 && cnt < 10 {
                    let size = (SLOWCIRCLE_SIZE + 1.0) * 2.0 * (1.0 - cnt as f32 / 10.0);
                    n.rotation.z = if sign > 0.0 { 360.0 } else { -360.0 };
                    n.scaling.x = size;
                    n.scaling.y = size;
                    n.scaling.z = 1.0;
                } else if cnt > 10 {
                    n.rotation.z = sign * (cnt as f32 * 4.0).to_radians();
                    n.scaling.x = SLOWCIRCLE_SIZE;
                    n.scaling.y = SLOWCIRCLE_SIZE;
                    n.scaling.z = 1.0;
                }
            }
        }
    }
}
/// Do title text's animation with counter.
pub fn script_title_text(manager: &mut EntityManager) {
    if let Some(ids) = manager.scripted_ids.get(type_name::<MarkerTitleText>()) {
        for id in ids {
            if let Some(counter) = manager.components.counters.get(id) {
                if let Some(n) = manager.components.texts.get_mut(id) {
                    n.rgba.w = (counter.count as f32).to_radians().cos().abs();
                }
            }
        }
    }
}
