use super::*;

pub fn create_frame(manager: &mut EntityManager) {
    let id = manager.create_entity(None);
    manager.components.sprites.insert(
        id,
        Sprite {
            visible: true,
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
}
