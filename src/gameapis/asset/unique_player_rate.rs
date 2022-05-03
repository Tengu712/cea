use super::*;

pub fn unique_player_rate(emngr: &mut EntityManager) {
    let player_id = if let Some(n) = emngr.unique_ids.get(UNIQUE_PLAYER) {
        *n
    } else {
        return;
    };
    let rate_id = if let Some(n) = emngr.unique_ids.get(UNIQUE_PLAYER_RATE) {
        *n
    } else {
        return;
    };
    if let Some(rate_text) = emngr.coms.texts.get_mut(&rate_id) {
        if let Some(player_pos) = emngr.coms.positions.get(&player_id) {
            rate_text.rect = Rect {
                l: SCREEN_WIDTH / 2.0 + player_pos.x + 20.0,
                r: SCREEN_WIDTH,
                t: SCREEN_HEIGHT / 2.0 - player_pos.y - 80.0,
                b: SCREEN_HEIGHT,
            };
        }
    }
}
