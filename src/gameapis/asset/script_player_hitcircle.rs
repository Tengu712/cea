use super::*;

pub struct MarkerPlayerHitCircle;

/// Change player hit circle's animation with input.
pub fn script_player_hitcircle(emngr: &mut EntityManager) {
    if let Some(ids) = emngr.scripted_ids.get(type_name::<MarkerPlayerHitCircle>()) {
        for id in ids {
            let cnt = emngr.input.s;
            if cnt > 0 {
                emngr.coms.sprites.active(id);
            } else {
                emngr.coms.sprites.disactive(id);
                continue;
            }
            if let Some(n) = emngr.coms.sprites.get_mut(id) {
                n.rotation.z = (cnt as f32 * 2.0).to_radians();
            }
        }
    }
}
