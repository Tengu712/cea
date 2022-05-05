use super::*;

impl Stage {
    /// Check hit. If snap, return 1. If gameover, activate gameover entity automatically.
    pub(super) fn check_hit(
        &mut self,
        emngr: &mut EntityManager,
        msg_hit: i64,
        msg_hit_fragile: i64,
    ) -> i64 {
        // Check snap delay is end
        let mut is_dead = false;
        if let Some(counter) = emngr.coms.counters.get(&self.snap_delay) {
            is_dead = counter.count == counter.count_max;
        }
        // Check death
        if msg_hit > 0 || is_dead {
            emngr.coms.counters.active(&self.relaunch_delay);
            if let Some(n) = emngr.coms.counters.get_mut(&self.relaunch_delay) {
                n.count = 0;
            }
            if self.die_player(emngr) {
                // If gameover.
                emngr.coms.counters.active(&self.gameover);
                emngr.remove_entity(&self.camera_lean);
                emngr.remove_entity(&self.stage);
            } else {
                // Still alive, reborn.
                self.spawn_player(emngr);
            }
        } else if emngr.coms.counters.get(&self.snap_delay).is_some() {
            // If down Z key during delay, player regain moving and shooting.
            if emngr.input.z == 1 {
                emngr.coms.counters.active(&self.player);
                emngr.coms.velocities.active(&self.player);
                emngr.coms.counters.disactive(&self.snap_delay);
                return 1;
            }
        } else if msg_hit_fragile > 0 {
            // If hit by fragile bullet, player cannot move and shoot.
            emngr.coms.counters.disactive(&self.player);
            emngr.coms.velocities.disactive(&self.player);
            // And delay is starting.
            emngr.coms.counters.active(&self.snap_delay);
            if let Some(n) = emngr.coms.counters.get_mut(&self.snap_delay) {
                n.count = 0;
            }
        }
        0
    }
}
