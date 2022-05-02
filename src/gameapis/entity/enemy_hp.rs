use super::*;

pub fn create_enemy_hp(manager: &mut EntityManager, hp_max: i64, enemy_id: EntityID) -> EntityID {
    let id = manager.create_entity();
    manager.insert_scripted_id(id, type_name::<MarkerGage>());
    manager.components.counters.insert(
        id,
        Counter {
            speed: 0,
            count: hp_max,
            count_max: hp_max,
        },
    );
    manager.components.positions.insert(
        id,
        Vector {
            x: 0.0,
            y: 0.0,
            z: Z_GAGE,
        },
    );
    manager
        .components
        .sameposition2ds
        .insert(id, SamePosition2D(enemy_id));
    manager.components.sprites.insert(
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
