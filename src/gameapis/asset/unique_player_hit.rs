use super::*;

pub fn unique_player_hit(manager: &mut EntityManager) {
    let id = if let Some(n) = manager.unique_ids.get(UNIQUE_PLAYER) {
        *n
    } else {
        return;
    };
    // Normal
    hit(manager, &id, TEAM_ENEMY_BULLET, MESSAGE_PLAYER_HIT, true);
    // Fragile
    hit(
        manager,
        &id,
        TEAM_ENEMY_BULLET_FRAGILE,
        MESSAGE_PLAYER_HIT_FRAGILE,
        true,
    );
    // Graze
    hit(
        manager,
        &id,
        TEAM_ENEMY_BULLET_GRAZE,
        MESSAGE_PLAYER_GRAZE,
        true,
    );
}
