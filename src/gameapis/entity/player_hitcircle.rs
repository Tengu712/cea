use super::*;

pub fn create_player_hitcircle(emngr: &mut EntityManager, player_id: EntityID) -> EntityID {
    let id = emngr.create_entity();
    emngr.insert_scripted_id(id, type_name::<MarkerPlayerHitCircle>());
    emngr.coms.positions.insert(
        id,
        Vector {
            x: 0.0,
            y: 0.0,
            z: Z_PLAYER_SLOW,
        },
    );
    emngr
        .coms
        .sameposition2ds
        .insert(id, SamePosition2D(player_id));
    emngr.coms.sprites.insert(
        id,
        Sprite {
            imgid: Some(IMGID_HITCIRCLE),
            scaling: Vector {
                x: 15.0,
                y: 15.0,
                z: 1.0,
            },
            color: COLOR_WHITE,
            ..Default::default()
        },
    );
    id
}
