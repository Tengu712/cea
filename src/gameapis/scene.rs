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
        create_fps(&mut world.manager);
        create_title_text(&mut world.manager);
        world.systems.push(system_update_counter);
        world.systems.push(system_fpsmeasure);
        world.systems.push(script_title_text);
        Box::new(Title {})
    }
}
impl Scene for Title {
    fn update(&mut self, world: &mut World) -> Option<Box<dyn Scene>> {
        if world.manager.input.z == 1 {
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
    pub delay_count: Option<EntityID>,
}
impl Stage {
    pub fn new(world: &mut World) -> Box<dyn Scene> {
        world.clear();
        world.manager.camera.rot.x = -30.0f32.to_radians();
        // entity
        let _ = create_fps(&mut world.manager);
        let _ = create_floor(&mut world.manager, 0);
        let _ = create_floor(&mut world.manager, 1);
        let _ = create_floor(&mut world.manager, 2);
        let _ = create_frame(&mut world.manager);
        let enemy = create_enemy(&mut world.manager);
        let e_hp = create_enemy_hp(&mut world.manager, 2000, enemy);
        let player = create_player(&mut world.manager);
        let player_slow1 = create_player_slow(&mut world.manager, player, true);
        let player_slow2 = create_player_slow(&mut world.manager, player, false);
        let rate = create_player_rate(&mut world.manager, player);
        let score = create_score(&mut world.manager, 0);
        let graze = create_graze(&mut world.manager, 0);
        let stage = create_stage1(&mut world.manager);
        let camera = create_camera(&mut world.manager);
        let camera_lean = create_camera_lean(&mut world.manager);
        // Unique
        world.manager.unique_ids.insert(UNIQUE_CAMERA, camera);
        world
            .manager
            .unique_ids
            .insert(UNIQUE_CAMERA_LEAN, camera_lean);
        world.manager.unique_ids.insert(UNIQUE_ENEMY, enemy);
        world.manager.unique_ids.insert(UNIQUE_PLAYER, player);
        world.manager.unique_ids.insert(UNIQUE_STAGE1, stage);
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
            delay_count: None,
        })
    }
}
impl Scene for Stage {
    fn update(&mut self, world: &mut World) -> Option<Box<dyn Scene>> {
        // Reserve message
        let msg_hit = world
            .manager
            .messages
            .remove(MESSAGE_PLAYER_HIT)
            .unwrap_or(0);
        let msg_hit_fragile = world
            .manager
            .messages
            .remove(MESSAGE_PLAYER_HIT_FRAGILE)
            .unwrap_or(0);
        let msg_graze = world
            .manager
            .messages
            .remove(MESSAGE_PLAYER_GRAZE)
            .unwrap_or(0);
        let msg_enemy_hit = world
            .manager
            .messages
            .remove(MESSAGE_ENEMY_HIT)
            .unwrap_or(0);
        // Delay
        let is_dead = if let Some(n) = self.delay_count {
            if let Some(counter) = world.manager.components.counters.get(&n) {
                counter.count == counter.count_max
            } else {
                false
            }
        } else {
            false
        };
        // Hit
        let mut is_snap = 0;
        if msg_hit > 0 || is_dead {
            world.manager.remove_entity(&self.player);
            world.manager.remove_entity(&self.player_slow1);
            world.manager.remove_entity(&self.player_slow2);
            world.manager.remove_entity(&self.rate);
            if let Some(n) = self.delay_count {
                world.manager.remove_entity(&n);
                self.delay_count = None;
            }
            self.player = create_player(&mut world.manager);
            self.player_slow1 = create_player_slow(&mut world.manager, self.player, true);
            self.player_slow2 = create_player_slow(&mut world.manager, self.player, false);
            self.rate = create_player_rate(&mut world.manager, self.player);
            world.manager.unique_ids.insert(UNIQUE_PLAYER, self.player);
        } else if let Some(n) = self.delay_count {
            // If down Z key during delay, player regain moving and shooting.
            if world.manager.input.z == 1 {
                world.manager.components.counters.active(&self.player);
                world.manager.components.velocities.active(&self.player);
                world.manager.remove_entity(&n);
                self.delay_count = None;
                is_snap = 1;
            }
        } else if msg_hit_fragile > 0 {
            // If hit by fragile bullet, player cannot move and shoot.
            world.manager.components.counters.disactive(&self.player);
            world.manager.components.velocities.disactive(&self.player);
            // And delay is starting.
            self.delay_count = Some(create_delay_count(&mut world.manager));
        }
        // Add rate
        let rate = if let Some(rate_counter) = world.manager.components.counters.get_mut(&self.rate)
        {
            let add = msg_graze * 10 + is_snap * 200;
            rate_counter.count = (rate_counter.count + add).min(rate_counter.count_max);
            (rate_counter.count as f32) / (rate_counter.count_max as f32) * 100.0
        } else {
            0.0
        };
        // Subtract enemy hp
        let (enemy_hp, enemy_hp_max) =
            if let Some(enemy_hp) = world.manager.components.counters.get_mut(&self.e_hp) {
                enemy_hp.count -= msg_enemy_hit;
                (enemy_hp.count, enemy_hp.count_max)
            } else {
                (0, 0)
            };
        // Add graze
        if let Some(graze_counter) = world.manager.components.counters.get_mut(&self.graze) {
            graze_counter.count += msg_graze;
            graze_counter.count_max += msg_graze;
        }
        // Add score
        if let Some(score_counter) = world.manager.components.counters.get_mut(&self.score) {
            let add = msg_graze * 30 + msg_enemy_hit * 10;
            score_counter.count += add;
            score_counter.count_max += add;
        }
        // Print console
        let (time_count, time_count_max) =
            if let Some(n) = world.manager.components.counters.get(&self.stage) {
                (n.count, n.count_max)
            } else {
                (0, 0)
            };
        println!("\x1b[2KTime : {} / {}", time_count, time_count_max);
        println!("\x1b[2KRate : {:.0} %", rate);
        println!(
            "\x1b[2KBulletNumber : {} / {}",
            world.manager.bullet_ids.len(),
            BULLET_MAX_NUM
        );
        println!("\x1b[2KEnemyHP : {} / {}", enemy_hp, enemy_hp_max);
        println!("\x1b[5A");
        None
    }
}
