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
    manager
        .components
        .positions
        .insert(id, Vector { x, y, z: Z_BULLET });
    manager
        .components
        .removerects
        .insert(id, BULLET_REMOVE_RECT);
    manager.components.collisions.insert(
        id,
        Collision {
            r: 100.0,
            team: TEAM_PLAYER_BULLET,
        },
    );
    manager.components.sprites.insert(
        id,
        Sprite {
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
