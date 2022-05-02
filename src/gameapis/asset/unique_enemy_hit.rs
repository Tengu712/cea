use super::*;

pub fn unique_enemy_hit(manager: &mut EntityManager) {
    let id = if let Some(n) = manager.unique_ids.get(UNIQUE_ENEMY) {
        *n
    } else {
        return;
    };
    hit(manager, &id, TEAM_PLAYER_BULLET, MESSAGE_ENEMY_HIT, true);
}
