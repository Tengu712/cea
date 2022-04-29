use super::*;

pub const PLAYER_KEY: EntityKey = "Player";

pub fn create_player(manager: &mut EntityManager) {
    let id = manager.create_entity(Some(PLAYER_KEY));
    manager
        .components
        .playeranimations
        .insert(id, PlayerAnimation::default());
    manager.components.playerinputs.insert(id, PlayerInput);
    manager.components.positions.insert(
        id,
        Position {
            x: 0.0,
            y: -280.0,
            z: 0.0,
        },
    );
    manager.components.restricts.insert(
        id,
        Rect3D {
            l: GAME_LEFT + 10.0,
            r: GAME_RIGHT - 10.0,
            t: GAME_TOP - 150.0,
            b: GAME_BOTTOM + 20.0,
            n: 0.0,
            f: 0.0,
        },
    );
    manager.components.sprites.insert(
        id,
        Sprite {
            visible: true,
            layer: LAYER_PLAYER,
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
        Velocity {
            direction: Vector::default(),
            speed: 8.0,
        },
    );
}
