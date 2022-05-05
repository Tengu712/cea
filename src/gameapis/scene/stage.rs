mod add_rate;
mod check_hit;
mod check_logue;
mod die_player;
pub mod new;
mod remove_bullets;
mod spawn_player;

pub use new::*;

use super::*;

#[derive(Default)]
pub struct Stage {
    player: EntityID,
    player_slow1: EntityID,
    player_slow2: EntityID,
    player_hitcircle: EntityID,
    score: EntityID,
    graze: EntityID,
    stage: EntityID,
    camera_lean: EntityID,
    e_hp: EntityID,
    p_hp: Vec<EntityID>,
    rate: EntityID,
    rate_delay: EntityID,
    snap_delay: EntityID,
    gameover: EntityID,
    logue: EntityID,
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
        let msg_bonus = world.emngr.messages.remove(MESSAGE_BONUS).unwrap_or(0);
        let msg_enemy_hit = world.emngr.messages.remove(MESSAGE_ENEMY_HIT).unwrap_or(0);
        // Check gameovered
        let is_gameovered = if let Some(n) = world.emngr.coms.counters.get(&self.gameover) {
            n.count == n.count_max
        } else {
            false
        };
        if is_gameovered {
            return Some(Title::new(world));
        }
        // Check logue
        self.check_logue(&mut world.emngr);
        // Check hit
        let is_snap = self.check_hit(world, msg_hit, msg_hit_fragile);
        // Add rate
        let rate = self.add_rate(world, msg_graze, msg_bonus, is_snap);
        // Subtraction of enemy hp and check defeat enemy
        let mut flg_move_phase = 0;
        if let Some(enemy_hp) = world.emngr.coms.counters.get_mut(&self.e_hp) {
            enemy_hp.count -= ((3.0 * rate as f32 / 1000.0 + 1.0) * 100.0) as i64 * msg_enemy_hit;
            if enemy_hp.count <= 0 {
                flg_move_phase = 1;
            }
        }
        // Check time up
        if let Some(stage_counter) = world.emngr.coms.counters.get(&self.stage) {
            if stage_counter.count >= stage_counter.count_max {
                flg_move_phase = 2;
            }
        }
        // Move phase
        if flg_move_phase > 0 {
            world.emngr.remove_entity(&self.e_hp);
            world.emngr.remove_entity(&self.stage);
            if flg_move_phase == 1 {
                self.remove_bullets(&mut world.emngr, true);
            } else {
                self.remove_bullets(&mut world.emngr, false);
            }
        }
        // Add graze
        let graze_count =
            if let Some(graze_counter) = world.emngr.coms.counters.get_mut(&self.graze) {
                graze_counter.count += msg_graze;
                graze_counter.count_max += msg_graze;
                graze_counter.count
            } else {
                0
            };
        // Add score
        if let Some(score_counter) = world.emngr.coms.counters.get_mut(&self.score) {
            let add = msg_graze * 30 + msg_enemy_hit * 10 + (1000 + 10 * graze_count / 2) * msg_bonus;
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
        println!(
            "\x1b[2KDamageExpect : {}",
            ((2.0 * rate as f32 / 1000.0 + 1.0) * 100.0) as i64
        );
        println!("\x1b[4A");
        None
    }
}
