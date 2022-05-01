use super::*;

pub fn create_player_bullet(manager: &mut EntityManager, x: f32, y: f32) -> EntityID {
    let id = manager.create_entity();
    manager.components.velocities.insert(
        id,
        Velocity {
            direction: Vector {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            speed: 40.0,
        },
    );
    manager.components.positions.insert(
        id,
        Vector {
            x: x,
            y: y,
            z: Z_BULLET,
        },
    );
    const BULLET_RECT: [f32; 4] = [
        GAME_LEFT - 80.0,
        GAME_RIGHT + 80.0,
        GAME_TOP + 80.0,
        GAME_BOTTOM - 80.0,
    ];
    manager.components.removerects.insert(
        id,
        RemoveRect {
            l: GAME_LEFT - 80.0,
            r: GAME_RIGHT,
            b: GAME_BOTTOM - 80.0,
            t: GAME_TOP + 80.0,
            n: -1000.0,
            f: 1000.0,
        },
    );
    manager.components.sprites.insert(
        id,
        Sprite {
            visible: true,
            imgid: Some(IMGID_BUL_FLAN),
            scaling: Vector {
                x: 90.0,
                y: 90.0,
                z: 1.0,
            },
            color: COLOR_WHITE,
            ..Default::default()
        },
    );
    id
}
