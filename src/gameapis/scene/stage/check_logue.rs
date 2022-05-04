use super::*;

impl Stage {
    pub(super) fn check_logue(&mut self, emngr: &mut EntityManager) {
        if let Some(counter) = emngr.coms.counters.get(&self.logue) {
            // If logue is end, start game.
            if counter.count == counter.count_max {
                emngr.coms.counters.active(&self.player);
                emngr.coms.counters.active(&self.stage);
                emngr.remove_entity(&self.logue);
            }
        }
    }
}
