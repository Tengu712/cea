mod convinient;
pub mod new;

pub use new::*;

use super::*;

#[derive(Default)]
pub struct Stage {
    pub player: EntityID,
    pub player_slow1: EntityID,
    pub player_slow2: EntityID,
    pub score: EntityID,
    pub graze: EntityID,
    pub stage: EntityID,
    pub e_hp: EntityID,
    pub rate: EntityID,
    pub rate_delay: EntityID,
    pub snap_delay: EntityID,
}
impl Scene for Stage {
    fn update(&mut self, world: &mut World) -> Option<Box<dyn Scene>> {
        // Reserve message
        let msg_hit = world.emngr.messages.remove(MESSAGE_PLAYER_HIT).unwrap_or(0);
        let msg_hit_fragile = world
            .emngr
            .messages
            .remove(MESSAGE_PLAYER_HIT_FRAGILE)
            .unwrap_or(0);
        let msg_graze = world
            .emngr
            .messages
            .remove(MESSAGE_PLAYER_GRAZE)
            .unwrap_or(0);
        let msg_enemy_hit = world.emngr.messages.remove(MESSAGE_ENEMY_HIT).unwrap_or(0);
        // Check hit
        let is_snap = self.check_hit(world, msg_hit, msg_hit_fragile);
        // Add rate
        self.add_rate(world, msg_graze, is_snap);
        // Subtraction of enemy hp
        if let Some(enemy_hp) = world.emngr.coms.counters.get_mut(&self.e_hp) {
            enemy_hp.count -= msg_enemy_hit;
        }
        // Add graze
        if let Some(graze_counter) = world.emngr.coms.counters.get_mut(&self.graze) {
            graze_counter.count += msg_graze;
            graze_counter.count_max += msg_graze;
        }
        // Add score
        if let Some(score_counter) = world.emngr.coms.counters.get_mut(&self.score) {
            let add = msg_graze * 30 + msg_enemy_hit * 10;
            score_counter.count += add;
            score_counter.count_max += add;
        }
        // Print console
        println!(
            "\x1b[2KBulletNumber : {} / {}",
            world.emngr.bullet_ids.len(),
            BULLET_MAX_NUM
        );
        println!(
            "\x1b[2KRate : {:.0} %",
            100.0
                * world
                    .emngr
                    .coms
                    .counters
                    .get(&self.rate)
                    .map(|n| n.count)
                    .unwrap_or(0) as f32
                / world
                    .emngr
                    .coms
                    .counters
                    .get(&self.rate)
                    .map(|n| n.count_max)
                    .unwrap_or(1) as f32
        );
        println!(
            "\x1b[2KEnemyHP : {} / {}",
            world
                .emngr
                .coms
                .counters
                .get(&self.e_hp)
                .map(|n| n.count)
                .unwrap_or(0),
            world
                .emngr
                .coms
                .counters
                .get(&self.e_hp)
                .map(|n| n.count_max)
                .unwrap_or(0),
        );
        println!("\x1b[4A");
        None
    }
}
