use super::*;

impl Stage {
    /// Check hit. If snap, return 1. If gameover, activate gameover entity automatically.
    pub(super) fn check_hit(
        &mut self,
        world: &mut World,
        msg_hit: i64,
        msg_hit_fragile: i64,
    ) -> i64 {
        // Check snap delay is end
        let mut is_dead = false;
        if let Some(counter) = world.emngr.coms.counters.get(&self.snap_delay) {
            is_dead = counter.count == counter.count_max;
        }
        // Check death
        if msg_hit > 0 || is_dead {
            if self.die_player(&mut world.emngr) {
                // If gameover.
                world.emngr.coms.counters.active(&self.gameover);
                world.emngr.remove_entity(&self.camera_lean);
                world.emngr.remove_entity(&self.stage);
            } else {
                // Still alive, reborn.
                self.spawn_player(&mut world.emngr);
            }
        } else if world.emngr.coms.counters.get(&self.snap_delay).is_some() {
            // If down Z key during delay, player regain moving and shooting.
            if world.emngr.input.z == 1 {
                world.emngr.coms.counters.active(&self.player);
                world.emngr.coms.velocities.active(&self.player);
                world.emngr.coms.counters.disactive(&self.snap_delay);
                return 1;
            }
        } else if msg_hit_fragile > 0 {
            // If hit by fragile bullet, player cannot move and shoot.
            world.emngr.coms.counters.disactive(&self.player);
            world.emngr.coms.velocities.disactive(&self.player);
            // And delay is starting.
            world.emngr.coms.counters.active(&self.snap_delay);
            if let Some(n) = world.emngr.coms.counters.get_mut(&self.snap_delay) {
                n.count = 0;
            }
        }
        0
    }
}
