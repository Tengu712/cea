use super::*;

impl Stage {
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
            world.emngr.remove_entity(&self.player);
            world.emngr.remove_entity(&self.player_slow1);
            world.emngr.remove_entity(&self.player_slow2);
            world.emngr.remove_entity(&self.player_hitcircle);
            world.emngr.remove_entity(&self.rate);
            world.emngr.coms.counters.disactive(&self.snap_delay);
            // Decrease player hp. Then, check gameover?
            if let Some(n) = self.p_hp.pop() {
                world.emngr.remove_entity(&n);
            } else {
                world.emngr.coms.counters.active(&self.gameover);
                world.emngr.remove_entity(&self.camera_lean);
                world.emngr.remove_entity(&self.stage);
                return 0;
            }
            self.player = create_player(&mut world.emngr);
            self.player_slow1 = create_player_slow(&mut world.emngr, self.player, true);
            self.player_slow2 = create_player_slow(&mut world.emngr, self.player, false);
            self.player_hitcircle = create_player_hitcircle(&mut world.emngr, self.player);
            self.rate = create_player_rate(&mut world.emngr);
            world.emngr.unique_ids.insert(UNIQUE_PLAYER, self.player);
            world.emngr.unique_ids.insert(UNIQUE_PLAYER_RATE, self.rate);
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
