use super::*;

impl Stage {
    pub(super) fn add_rate(
        &mut self,
        world: &mut World,
        msg_graze: i64,
        msg_bonus: i64,
        is_snap: i64,
    ) -> i64 {
        let mut add_rate = msg_graze * 10 + is_snap * 200 + msg_bonus * 2;
        if let Some(n) = world.emngr.coms.counters.get_mut(&self.rate_delay) {
            // If graze or snap, natural decrease count is set 0.
            if add_rate > 0 {
                n.count = 0;
            }
            // Or, natural decrease. Moreover, rate will decrease while shooting.
            else {
                add_rate -= (n.count == n.count_max) as i64 + (world.emngr.input.z > 0) as i64 * 2;
            }
        }
        // If down Z key when not snapping, rate will decrease.
        if is_snap != 1 && world.emngr.input.z == 1 {
            add_rate -= 100;
        }
        if let Some(rate_counter) = world.emngr.coms.counters.get_mut(&self.rate) {
            rate_counter.count = (rate_counter.count + add_rate)
                .min(rate_counter.count_max)
                .max(0);
            rate_counter.count
        } else {
            0
        }
    }
}
