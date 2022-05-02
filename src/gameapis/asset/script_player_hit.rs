use super::*;

pub struct MarkerPlayerHit;

pub fn script_player_hit(manager: &mut EntityManager) {
    let mut ids = Vec::new();
    if let Some(n) = manager.scripted_ids.get_mut(type_name::<MarkerPlayerHit>()) {
        for id in n.iter() {
            ids.push(*id);
        }
    }
    for id in ids {
        hit(
            manager,
            &id,
            TEAM_ENEMY_BULLET,
            MESSAGE_PLAYER_HIT,
            MESSAGE_PLAYER_GRAZE,
        );
    }
}
