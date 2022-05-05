use super::*;

impl Stage {
    pub(super) fn spawn_logue(&mut self, emngr: &mut EntityManager) {
        self.logue = create_logue_text(emngr);
        self.logue_left = create_logue_image(emngr);
        self.logue_right = create_logue_image(emngr);
        emngr.unique_ids.insert(UNIQUE_LOGUE1, self.logue);
        emngr.unique_ids.insert(UNIQUE_LOGUE_LEFT, self.logue_left);
        emngr
            .unique_ids
            .insert(UNIQUE_LOGUE_RIGHT, self.logue_right);
    }
    pub(super) fn die_logue(&mut self, emngr: &mut EntityManager) {
        emngr.remove_entity(&self.logue);
        emngr.remove_entity(&self.logue_left);
        emngr.remove_entity(&self.logue_right);
    }
    pub(super) fn check_logue(&mut self, emngr: &mut EntityManager) {
        if let Some(counter) = emngr.coms.counters.get(&self.logue) {
            // If logue is end, start game.
            if counter.count == counter.count_max {
                self.die_logue(emngr);
                emngr.coms.counters.active(&self.player);
                emngr.coms.counters.active(&self.stage);
            }
        }
    }
}
