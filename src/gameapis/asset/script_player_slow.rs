use super::*;

const SLOWCIRCLE_SIZE: f32 = 140.0;

pub struct MarkerPlayerSlow;

/// Change player slow circle's animation with input.
pub fn script_player_slow(manager: &mut EntityManager) {
    if let Some(ids) = manager.scripted_ids.get(type_name::<MarkerPlayerSlow>()) {
        for id in ids {
            let cnt = manager.input.s;
            if cnt > 0 {
                manager.components.sprites.active(id);
            } else {
                manager.components.sprites.disactive(id);
                continue;
            }
            if let Some(n) = manager.components.sprites.get_mut(id) {
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
