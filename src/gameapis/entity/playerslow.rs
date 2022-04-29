use super::*;

pub fn create_player_slow(manager: &mut EntityManager) {
    let id = manager.create_entity(None);
    manager.components.positions.insert(id, Position::default());
    manager
        .components
        .samepositions
        .insert(id, SamePosition(PLAYER_KEY));
    manager.components.sprites.insert(
        id,
        Sprite {
            layer: LAYER_PLAYER_SLOW,
            imgid: Some(IMGID_SLOWCIRCLE),
            scaling: Vector {
                x: 140.0,
                y: 140.0,
                z: 1.0,
            },
            color: COLOR_WHITE,
            ..Default::default()
        },
    );
}
