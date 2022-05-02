use super::*;

pub fn create_player_bullet(emngr: &mut EntityManager, x: f32, y: f32) -> EntityID {
    let id = emngr.create_entity();
    emngr.coms.velocities.insert(
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
    emngr
        .coms
        .positions
        .insert(id, Vector { x, y, z: Z_BULLET });
    emngr
        .coms
        .removerects
        .insert(id, BULLET_REMOVE_RECT);
    emngr.coms.collisions.insert(
        id,
        Collision {
            r: 100.0,
            team: TEAM_PLAYER_BULLET,
        },
    );
    emngr.coms.sprites.insert(
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
