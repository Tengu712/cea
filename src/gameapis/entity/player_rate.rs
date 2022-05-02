use super::*;

pub fn create_player_rate(emngr: &mut EntityManager, player_id: EntityID) -> EntityID {
    let id = emngr.create_entity();
    emngr.insert_scripted_id(id, type_name::<MarkerGage>());
    emngr.coms.counters.insert(
        id,
        Counter {
            speed: 0,
            count: 0,
            count_max: 1000,
        },
    );
    emngr.coms.positions.insert(
        id,
        Vector {
            x: 0.0,
            y: 0.0,
            z: Z_GAGE,
        },
    );
    emngr
        .coms
        .sameposition2ds
        .insert(id, SamePosition2D(player_id));
    emngr.coms.sprites.insert(
        id,
        Sprite {
            imgid: Some(IMGID_RATE),
            scaling: Vector {
                x: 160.0,
                y: 160.0,
                z: Z_GAGE,
            },
            color: COLOR_WHITE,
            ..Default::default()
        },
    );
    id
}
