use super::*;

/// Change velocity with input. The change is specify to player.
pub fn system_playerinput(manager: &mut EntityManager) {
    for k in manager.components.playerinputs.keys() {
        if let Some(n) = manager.components.velocities.get_mut(k) {
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
/// Do player's animation. Based on player's velocity.
pub fn system_playeranimation(manager: &mut EntityManager) {
    for k in manager.components.playeranimations.keys() {
        if let Some(velocity) = manager.components.velocities.get(k) {
            if let Some(n) = manager.components.sprites.get_mut(k) {
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
/// Change player slow circle's animation. Based on left shift input.
pub fn system_playerslowanimation(manager: &mut EntityManager) {
    const SLOWCIRCLE_SIZE: f32 = 140.0;
    for (k, v) in manager.components.playerslowanimations.iter_mut() {
        if let Some(n) = manager.components.sprites.get_mut(k) {
            let cnt = manager.input.s;
            n.visible = cnt > 0;
            if cnt > 0 && cnt < 10 {
                let size = (SLOWCIRCLE_SIZE + 1.0) * 2.0 * (1.0 - cnt as f32 / 10.0);
                n.rotation.z = 0.0;
                n.scaling.x = size;
                n.scaling.y = size;
                n.scaling.z = 1.0;
            } else {
                let coef = if v.0 { 1.0 } else { -1.0 };
                n.rotation.z = coef * (cnt as f32 * 4.0).to_radians();
                n.scaling.x = SLOWCIRCLE_SIZE;
                n.scaling.y = SLOWCIRCLE_SIZE;
                n.scaling.z = 1.0;
            }
        }
    }
}
