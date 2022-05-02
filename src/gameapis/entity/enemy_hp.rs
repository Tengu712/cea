use super::*;

pub fn create_enemy_hp(emngr: &mut EntityManager, hp_max: i64, enemy_id: EntityID) -> EntityID {
    let id = emngr.create_entity();
    emngr.insert_scripted_id(id, type_name::<MarkerGage>());
    emngr.coms.counters.insert(
        id,
        Counter {
            speed: 0,
            count: hp_max,
            count_max: hp_max,
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
        .insert(id, SamePosition2D(enemy_id));
    emngr.coms.sprites.insert(
        id,
        Sprite {
            imgid: Some(IMGID_HP),
            scaling: Vector {
                x: 280.0,
                y: 280.0,
                z: Z_GAGE,
            },
            color: COLOR_WHITE,
            ..Default::default()
        },
    );
    id
}
