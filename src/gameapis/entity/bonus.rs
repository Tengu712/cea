use super::*;

pub fn create_bonus(emngr: &mut EntityManager, x: f32, y: f32) -> EntityID {
    let id = emngr.create_entity();
    emngr.insert_scripted_id(id, type_name::<MarkerBonus>());
    emngr
        .coms
        .positions
        .insert(id, Vector { x, y, z: Z_BULLET });
    emngr.coms.velocities.insert(id, Velocity::default());
    emngr.coms.sprites.insert(
        id,
        Sprite {
            imgid: None,
            scaling: Vector {
                x: 20.0,
                y: 20.0,
                z: 1.0,
            },
            color: COLOR_WHITE,
            ..Default::default()
        },
    );
    emngr.coms.collisions.insert(
        id,
        Collision {
            r: 20.0,
            team: TEAM_BONUS,
        },
    );
    id
}
