use super::*;

pub struct MarkerPlayer;

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
                if manager.input.s > 0 {
                    n.speed = 4.0;
                } else {
                    n.speed = 8.0;
                }
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
