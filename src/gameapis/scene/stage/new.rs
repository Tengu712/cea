use super::*;

impl Stage {
    pub fn new(world: &mut World) -> Box<dyn Scene> {
        world.clear();
        world.emngr.camera.rot.x = -30.0f32.to_radians();
        // entity
        let _ = create_fps(&mut world.emngr);
        for i in 0..3 {
            let _ = create_floor(&mut world.emngr, i);
            for j in 0..2 {
                let _ = create_bg_decolation(&mut world.emngr, i, j, false);
                let _ = create_bg_decolation(&mut world.emngr, i, j, true);
            }
        }
        let _ = create_frame(&mut world.emngr);
        let enemy = create_enemy(&mut world.emngr);
        let e_hp = create_enemy_hp(&mut world.emngr, 30000);
        let player = create_player(&mut world.emngr);
        let player_slow1 = create_player_slow(&mut world.emngr, player, true);
        let player_slow2 = create_player_slow(&mut world.emngr, player, false);
        let player_hitcircle = create_player_hitcircle(&mut world.emngr, player);
        let rate = create_player_rate(&mut world.emngr);
        let rate_delay = create_delay_count(&mut world.emngr, 60);
        let snap_delay = create_delay_count(&mut world.emngr, 10);
        let score = create_score(&mut world.emngr, 0);
        let graze = create_graze(&mut world.emngr, 0);
        let stage = create_stage1(&mut world.emngr);
        let camera = create_camera(&mut world.emngr);
        let camera_lean = create_camera_lean(&mut world.emngr);
        let gameover = create_gameover(&mut world.emngr);
        let mut p_hp = Vec::new();
        for i in 0..2 {
            p_hp.push(create_player_hp(&mut world.emngr, i));
        }
        world.emngr.coms.counters.disactive(&snap_delay);
        world.emngr.coms.counters.disactive(&gameover);
        // Unique
        world.emngr.unique_ids.insert(UNIQUE_CAMERA, camera);
        world
            .emngr
            .unique_ids
            .insert(UNIQUE_CAMERA_LEAN, camera_lean);
        world.emngr.unique_ids.insert(UNIQUE_ENEMY, enemy);
        world.emngr.unique_ids.insert(UNIQUE_PLAYER, player);
        world.emngr.unique_ids.insert(UNIQUE_PLAYER_RATE, rate);
        world.emngr.unique_ids.insert(UNIQUE_STAGE1, stage);
        // script
        world.systems.push(unique_stage1_1);
        world.systems.push(unique_camera);
        world.systems.push(unique_camera_lean);
        world.systems.push(unique_player);
        world.systems.push(unique_enemy_hit);
        world.systems.push(unique_player_hit);
        world.systems.push(unique_player_shot);
        world.systems.push(unique_player_rate);
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
        Box::new(Stage {
            player,
            player_slow1,
            player_slow2,
            player_hitcircle,
            score,
            graze,
            stage,
            camera_lean,
            e_hp,
            p_hp,
            rate,
            rate_delay,
            snap_delay,
            gameover,
        })
    }
}
