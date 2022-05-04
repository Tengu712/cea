use super::*;

pub fn unique_player_hit(emngr: &mut EntityManager) {
    let id = if let Some(n) = emngr.unique_ids.get(UNIQUE_PLAYER) {
        *n
    } else {
        return;
    };
    // Normal
    hit(emngr, &id, TEAM_ENEMY_BULLET, MESSAGE_PLAYER_HIT, true);
    // Fragile
    hit(
        emngr,
        &id,
        TEAM_ENEMY_BULLET_FRAGILE,
        MESSAGE_PLAYER_HIT_FRAGILE,
        true,
    );
    // Graze
    hit(
        emngr,
        &id,
        TEAM_ENEMY_BULLET_GRAZE,
        MESSAGE_PLAYER_GRAZE,
        true,
    );
    // Bonus
    hit(emngr, &id, TEAM_BONUS, MESSAGE_BONUS, true);
}
