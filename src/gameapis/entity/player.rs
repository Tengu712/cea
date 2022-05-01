use super::*;

pub fn create_player(manager: &mut EntityManager) {
    let id = manager.create_entity();
    manager.insert_scripted_id(id, type_name::<MarkerPlayer>());
    manager.components.positions.insert(
        id,
        true,
        Position {
            x: 0.0,
            y: -280.0,
            z: Z_PLAYER,
        },
    );
    manager.components.restricts.insert(
        id,
        true,
        Rect3D {
            l: GAME_LEFT + 10.0,
            r: GAME_RIGHT - 10.0,
            t: GAME_TOP - 150.0,
            b: GAME_BOTTOM + 20.0,
            n: -1000.0,
            f: 1000.0,
        },
    );
    manager.components.sprites.insert(
        id,
        true,
        Sprite {
            visible: true,
            imgid: Some(IMGID_FLAN_B0),
            scaling: Vector {
                x: 100.0,
                y: 100.0,
                z: 1.0,
            },
            color: COLOR_WHITE,
            ..Default::default()
        },
    );
    manager.components.velocities.insert(
        id,
        true,
        Velocity {
            direction: Vector::default(),
            speed: 8.0,
        },
    );
}

pub fn create_player_slow(manager: &mut EntityManager, flg: bool) {
    let id = manager.create_entity();
    manager.insert_scripted_id(id, type_name::<MarkerPlayerSlow>());
    manager.components.positions.insert(
        id,
        true,
        Position {
            x: 0.0,
            y: 0.0,
            z: Z_PLAYER_SLOW,
        },
    );
    manager.components.sprites.insert(
        id,
        true,
        Sprite {
            visible: true,
            imgid: Some(IMGID_SLOWCIRCLE),
            color: COLOR_WHITE,
            ..Default::default()
        },
    );
}
