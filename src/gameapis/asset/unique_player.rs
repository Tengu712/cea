use super::*;

/// Change player's velocity with input. Then, it does player's animation with input.
pub fn unique_player(emngr: &mut EntityManager) {
    let id = if let Some(n) = emngr.unique_ids.get(UNIQUE_PLAYER) {
        *n
    } else {
        return;
    };
    let mut lr = 0 as i32;
    // Velocity
    if let Some(n) = emngr.coms.velocities.get_mut(&id) {
        lr = (emngr.input.right > 0) as i32 - (emngr.input.left > 0) as i32;
        let ud = (emngr.input.up > 0) as i32 - (emngr.input.down > 0) as i32;
        let coef = if lr.abs() + ud.abs() == 2 {
            1.0 / std::f32::consts::SQRT_2
        } else {
            1.0
        };
        n.direction.x = lr as f32 * coef;
        n.direction.y = ud as f32 * coef;
        if emngr.input.s > 0 {
            n.speed = 4.0;
        } else {
            n.speed = 8.0;
        }
    }
    // Reflect on Sprite
    if let Some(n) = emngr.coms.sprites.get_mut(&id) {
        if lr > 0 {
            n.imgid = Some(IMGID_FLAN_R0);
        } else if lr < 0 {
            n.imgid = Some(IMGID_FLAN_L0);
        } else {
            n.imgid = Some(IMGID_FLAN_B0);
        }
    }
}
