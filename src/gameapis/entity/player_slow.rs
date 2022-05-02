use super::*;

pub fn create_player_slow(emngr: &mut EntityManager, player_id: EntityID, flg: bool) -> EntityID {
    let id = emngr.create_entity();
    emngr.insert_scripted_id(id, type_name::<MarkerPlayerSlow>());
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
            imgid: Some(IMGID_SLOWCIRCLE),
            color: COLOR_WHITE,
            rotation: Vector {
                x: 0.0,
                y: 0.0,
                z: if flg { 360.0 } else { -360.0 },
            },
            ..Default::default()
        },
    );
    id
}
