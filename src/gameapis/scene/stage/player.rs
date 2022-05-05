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
    /// Remove player's entities. If gameover, return true.
    pub(super) fn die_player(&mut self, emngr: &mut EntityManager) -> bool {
        emngr.remove_entity(&self.player);
        emngr.remove_entity(&self.player_slow1);
        emngr.remove_entity(&self.player_slow2);
        emngr.remove_entity(&self.player_hitcircle);
        emngr.remove_entity(&self.rate);
        emngr.coms.counters.disactive(&self.snap_delay);
        // Remove bullets
        self.remove_bullets(emngr, false);
        // Decrease player hp. Then, check gameover?
        if let Some(n) = self.p_hp.pop() {
            emngr.remove_entity(&n);
            false
        } else {
            true
        }
    }
}
