use super::*;

pub fn unique_enemy_hit(emngr: &mut EntityManager) {
    let id = if let Some(n) = emngr.unique_ids.get(UNIQUE_ENEMY) {
        *n
    } else {
        return;
    };
    hit(emngr, &id, TEAM_PLAYER_BULLET, MESSAGE_ENEMY_HIT, true);
}
