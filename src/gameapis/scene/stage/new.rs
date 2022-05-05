use super::*;

impl Stage {
    pub fn new(world: &mut World) -> Box<dyn Scene> {
        world.clear();
        world.emngr.camera.rot.x = -30.0f32.to_radians();
        let mut stage = Stage::default();
        // player
        stage.spawn_player(&mut world.emngr);
        world.emngr.coms.counters.disactive(&stage.player);
        // hp
        stage.e_hp = create_enemy_hp(&mut world.emngr, 30000);
        for i in 0..2 {
            stage.p_hp.push(create_player_hp(&mut world.emngr, i));
        }
        // delay
        stage.rate_delay = create_delay_count(&mut world.emngr, 60);
        stage.snap_delay = create_delay_count(&mut world.emngr, 10);
        stage.relaunch_delay = create_delay_count(&mut world.emngr, 60);
        world.emngr.coms.counters.disactive(&stage.snap_delay);
        world.emngr.coms.counters.disactive(&stage.relaunch_delay);
        // value
        stage.score = create_score(&mut world.emngr, 0);
        stage.graze = create_graze(&mut world.emngr, 0);
        // logue
        stage.spawn_logue(&mut world.emngr);
        // Other
        stage.stage = create_stage1(&mut world.emngr);
        stage.gameover = create_gameover(&mut world.emngr);
        stage.camera_lean = create_camera_lean(&mut world.emngr);
        world.emngr.coms.counters.disactive(&stage.stage);
        world.emngr.coms.counters.disactive(&stage.gameover);
        world.emngr.unique_ids.insert(UNIQUE_STAGE1, stage.stage);
        world
            .emngr
            .unique_ids
            .insert(UNIQUE_CAMERA_LEAN, stage.camera_lean);
        // Unique but keep object
        let enemy = create_enemy(&mut world.emngr);
        let camera = create_camera(&mut world.emngr);
        world.emngr.unique_ids.insert(UNIQUE_ENEMY, enemy);
        world.emngr.unique_ids.insert(UNIQUE_CAMERA, camera);
        // Common object
        let _ = create_fps(&mut world.emngr);
        let _ = create_frame(&mut world.emngr);
        for i in 0..3 {
            let _ = create_floor(&mut world.emngr, i);
            for j in 0..2 {
                let _ = create_bg_decolation(&mut world.emngr, i, j, false);
                let _ = create_bg_decolation(&mut world.emngr, i, j, true);
            }
        }
        // script
        world.systems.push(unique_stage1_1);
        world.systems.push(unique_logue1);
        world.systems.push(unique_camera);
        world.systems.push(unique_camera_lean);
        world.systems.push(unique_player);
        world.systems.push(unique_enemy_hit);
        world.systems.push(unique_player_hit);
        world.systems.push(unique_player_shot);
        world.systems.push(unique_player_rate);
        world.systems.push(script_bonus);
        world.systems.push(script_player_hitcircle);
        world.systems.push(script_player_slow);
        // system
        world.systems.push(system_fpsmeasure);
        world.systems.push(system_update_counter);
        world.systems.push(system_velocity_position);
        world.systems.push(system_remove_rect);
        world.systems.push(system_restrict_position);
        world.systems.push(system_same_position_2d);
        world.systems.push(system_position_sprite);
        world.systems.push(system_value_sprite);
        world.systems.push(system_value_text);
        // Finish
        Box::new(stage)
    }
}
