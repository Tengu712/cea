use super::*;

impl Stage {
    pub(super) fn spawn_player(&mut self, emngr: &mut EntityManager) {
        self.player = create_player(emngr);
        self.player_slow1 = create_player_slow(emngr, self.player, true);
        self.player_slow2 = create_player_slow(emngr, self.player, false);
        self.player_hitcircle = create_player_hitcircle(emngr, self.player);
        self.rate = create_player_rate(emngr);
        emngr.unique_ids.insert(UNIQUE_PLAYER, self.player);
        emngr.unique_ids.insert(UNIQUE_PLAYER_RATE, self.rate);
    }
}
