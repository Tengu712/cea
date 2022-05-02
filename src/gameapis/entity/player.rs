use super::*;

pub fn create_player(manager: &mut EntityManager) -> EntityID {
    let id = manager.create_entity();
    manager.insert_scripted_id(id, type_name::<MarkerPlayer>());
    manager.insert_scripted_id(id, type_name::<MarkerPlayerHit>());
    manager.insert_scripted_id(id, type_name::<MarkerPlayerShot>());
    manager.components.counters.insert(
        id,
        Counter {
            speed: 1,
            count: 0,
            count_max: std::u64::MAX,
        },
    );
    manager.components.positions.insert(
        id,
        Vector {
            x: 0.0,
            y: -280.0,
            z: Z_PLAYER,
        },
    );
    manager.components.restricts.insert(
        id,
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
        Sprite {
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
    manager
        .components
        .velocities
        .insert(id, Velocity::default());
    id
}
