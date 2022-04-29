use super::*;

pub fn create_player_slow(components: &mut Components) {
    components
        .positions
        .insert(components.next_entity_id, Position::default());
    components.samepositions.insert(
        components.next_entity_id,
        SamePosition {
            targetkey: PLAYER_KEY,
            targetposition: Position::default(),
        },
    );
    components.sprites.insert(
        components.next_entity_id,
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
    components.next_entity_id += 1;
}
