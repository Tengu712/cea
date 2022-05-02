use super::*;

pub fn create_player_rate(manager: &mut EntityManager, player_id: EntityID) -> EntityID {
    let id = manager.create_entity();
    manager.insert_scripted_id(id, type_name::<MarkerGage>());
    manager.components.counters.insert(
        id,
        Counter {
            speed: 0,
            count: 0,
            count_max: 1000,
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
        .insert(id, SamePosition2D(player_id));
    manager.components.sprites.insert(
        id,
        Sprite {
            imgid: Some(IMGID_HP),
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
