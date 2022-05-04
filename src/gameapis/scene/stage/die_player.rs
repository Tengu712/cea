use super::*;

impl Stage {
    /// Remove player's entities. If gameover, return true.
    pub(super) fn die_player(&mut self, emngr: &mut EntityManager) -> bool {
        emngr.remove_entity(&self.player);
        emngr.remove_entity(&self.player_slow1);
        emngr.remove_entity(&self.player_slow2);
        emngr.remove_entity(&self.player_hitcircle);
        emngr.remove_entity(&self.rate);
        emngr.coms.counters.disactive(&self.snap_delay);
        // Remove bullets
        self.remove_bullets(emngr);
        // Decrease player hp. Then, check gameover?
        if let Some(n) = self.p_hp.pop() {
            emngr.remove_entity(&n);
            false
        } else {
            true
        }
    }
}
