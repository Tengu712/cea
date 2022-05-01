use super::*;

pub fn create_player_slow(manager: &mut EntityManager, player_id: EntityID, flg: bool) -> EntityID {
    let id = manager.create_entity();
    manager.insert_scripted_id(id, type_name::<MarkerPlayerSlow>());
    manager.components.positions.insert(
        id,
        Position {
            x: 0.0,
            y: 0.0,
            z: Z_PLAYER_SLOW,
        },
    );
    manager
        .components
        .sameposition2ds
        .insert(id, SamePosition2D(player_id));
    manager.components.sprites.insert(
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
