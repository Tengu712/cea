use super::asset::*;
use super::entity::*;
use super::system::*;
use super::*;

pub trait Scene {
    fn update(&mut self, world: &mut World) -> Option<Box<dyn Scene>>;
}

#[derive(Default)]
pub struct Title {}
impl Title {
    pub fn new(world: &mut World) -> Box<dyn Scene> {
        world.clear();
        create_fps(&mut world.emngr);
        create_title_text(&mut world.emngr);
        world.systems.push(system_update_counter);
        world.systems.push(system_fpsmeasure);
        world.systems.push(script_title_text);
        Box::new(Title {})
    }
}
impl Scene for Title {
    fn update(&mut self, world: &mut World) -> Option<Box<dyn Scene>> {
        if world.emngr.input.z == 1 {
            Some(Stage::new(world))
        } else {
            None
        }
    }
}

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
impl Stage {
    pub fn new(world: &mut World) -> Box<dyn Scene> {
        world.clear();
        world.emngr.camera.rot.x = -30.0f32.to_radians();
        // entity
        let _ = create_fps(&mut world.emngr);
        let _ = create_floor(&mut world.emngr, 0);
        let _ = create_floor(&mut world.emngr, 1);
        let _ = create_floor(&mut world.emngr, 2);
        let _ = create_frame(&mut world.emngr);
        let enemy = create_enemy(&mut world.emngr);
        let e_hp = create_enemy_hp(&mut world.emngr, 2000, enemy);
        let player = create_player(&mut world.emngr);
        let player_slow1 = create_player_slow(&mut world.emngr, player, true);
        let player_slow2 = create_player_slow(&mut world.emngr, player, false);
        let rate = create_player_rate(&mut world.emngr, player);
        let rate_delay = create_delay_count(&mut world.emngr, 60);
        let snap_delay = create_delay_count(&mut world.emngr, 10);
        let score = create_score(&mut world.emngr, 0);
        let graze = create_graze(&mut world.emngr, 0);
        let stage = create_stage1(&mut world.emngr);
        let camera = create_camera(&mut world.emngr);
        let camera_lean = create_camera_lean(&mut world.emngr);
        // Unique
        world.emngr.unique_ids.insert(UNIQUE_CAMERA, camera);
        world
            .emngr
            .unique_ids
            .insert(UNIQUE_CAMERA_LEAN, camera_lean);
        world.emngr.unique_ids.insert(UNIQUE_ENEMY, enemy);
        world.emngr.unique_ids.insert(UNIQUE_PLAYER, player);
        world.emngr.unique_ids.insert(UNIQUE_STAGE1, stage);
        // script
        world.systems.push(unique_stage1_1);
        world.systems.push(unique_camera);
        world.systems.push(unique_camera_lean);
        world.systems.push(unique_player);
        world.systems.push(unique_enemy_hit);
        world.systems.push(unique_player_hit);
        world.systems.push(unique_player_shot);
        world.systems.push(script_player_slow);
        world.systems.push(script_gage);
        // system
        world.systems.push(system_fpsmeasure);
        world.systems.push(system_update_counter);
        world.systems.push(system_velocity_position);
        world.systems.push(system_remove_rect);
        world.systems.push(system_restrict_position);
        world.systems.push(system_same_position_2d);
        world.systems.push(system_position_sprite);
        world.systems.push(system_value_text);
        Box::new(Stage {
            player,
            player_slow1,
            player_slow2,
            score,
            graze,
            stage,
            e_hp,
            rate,
            rate_delay,
            snap_delay,
        })
    }
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
        // Check snap delay is end
        let is_dead = if let Some(counter) = world.emngr.coms.counters.get(&self.snap_delay) {
            counter.count == counter.count_max
        } else {
            false
        };
        // Hit
        let mut is_snap = 0;
        if msg_hit > 0 || is_dead {
            world.emngr.remove_entity(&self.player);
            world.emngr.remove_entity(&self.player_slow1);
            world.emngr.remove_entity(&self.player_slow2);
            world.emngr.remove_entity(&self.rate);
            world.emngr.coms.counters.disactive(&self.snap_delay);
            self.player = create_player(&mut world.emngr);
            self.player_slow1 = create_player_slow(&mut world.emngr, self.player, true);
            self.player_slow2 = create_player_slow(&mut world.emngr, self.player, false);
            self.rate = create_player_rate(&mut world.emngr, self.player);
            world.emngr.unique_ids.insert(UNIQUE_PLAYER, self.player);
        } else if world.emngr.coms.counters.get(&self.snap_delay).is_some() {
            // If down Z key during delay, player regain moving and shooting.
            if world.emngr.input.z == 1 {
                world.emngr.coms.counters.active(&self.player);
                world.emngr.coms.velocities.active(&self.player);
                world.emngr.coms.counters.disactive(&self.snap_delay);
                is_snap = 1;
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
        // Addition of rate
        let mut add_rate = msg_graze * 10 + is_snap * 200;
        if let Some(n) = world.emngr.coms.counters.get_mut(&self.rate_delay) {
            if add_rate > 0 {
                n.count = 0;
            } else {
                add_rate -= (n.count == n.count_max) as i64 + (world.emngr.input.z > 0) as i64;
            }
        }
        if let Some(rate_counter) = world.emngr.coms.counters.get_mut(&self.rate) {
            rate_counter.count = (rate_counter.count + add_rate).min(rate_counter.count_max);
        }
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
