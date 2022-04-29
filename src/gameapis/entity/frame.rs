use super::*;

pub fn create_frame(components: &mut Components) {
    components.sprites.insert(
        components.next_entity_id,
        Sprite {
            layer: LAYER_FRAME,
            imgid: Some(IMGID_FRAME),
            scaling: Vector {
                x: SCREEN_WIDTH,
                y: SCREEN_HEIGHT,
                z: 1.0,
            },
            color: COLOR_WHITE,
            ..Default::default()
        },
    );
    components.next_entity_id += 1;
}
