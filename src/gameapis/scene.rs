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
    pub score: EntityID,
    pub graze: EntityID,
    pub phase: EntityID,
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
        let _ = create_enemy(&mut world.manager);
        let player = create_player(&mut world.manager);
        let _ = create_player_slow(&mut world.manager, player, true);
        let _ = create_player_slow(&mut world.manager, player, false);
        let _ = create_frame(&mut world.manager);
        let score = create_score(&mut world.manager, 0);
        let graze = create_graze(&mut world.manager, 0);
        let phase = create_script_1_1(&mut world.manager);
        let _ = create_script_camera(&mut world.manager);
        let _ = create_script_camera_lean(&mut world.manager);
        // script
        world.systems.push(script_1_1);
        world.systems.push(script_camera);
        world.systems.push(script_camera_lean);
        world.systems.push(script_player);
        world.systems.push(script_player_slow);
        world.systems.push(script_player_shot);
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
            score,
            graze,
            phase,
        })
    }
}
impl Scene for Stage {
    fn update(&mut self, world: &mut World) -> Option<Box<dyn Scene>> {
        print!("\r\x1b[2KBulletNumber : {}", world.manager.bullet_ids.len());
        None
    }
}
