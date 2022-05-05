use super::*;

pub struct MarkerBonus;

pub fn script_bonus(emngr: &mut EntityManager) {
    if let Some(ids) = emngr.scripted_ids.get(type_name::<MarkerBonus>()) {
        for id in ids {
            if let Some(velocity) = emngr.coms.velocities.get_mut(id) {
                velocity.direction.x = 0.0;
                velocity.direction.y = -1.0;
                velocity.speed = 10.0;
                let player_pos = if let Some(player_id) = emngr.unique_ids.get(UNIQUE_PLAYER) {
                    if let Some(player_pos) = emngr.coms.positions.get(player_id) {
                        player_pos.clone()
                    } else {
                        continue;
                    }
                } else {
                    continue;
                };
                if let Some(position) = emngr.coms.positions.get_mut(id) {
                    let dx = player_pos.x - position.x;
                    let dy = player_pos.y - position.y;
                    if dx != 0.0 || dy != 0.0 {
                        let mag = (dx.powf(2.0) + dy.powf(2.0)).powf(0.5);
                        velocity.direction.x = dx / mag;
                        velocity.direction.y = dy / mag;
                    }
                    velocity.speed = 15.0;
                }
            }
        }
    }
}
