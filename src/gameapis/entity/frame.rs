use super::*;

pub fn create_frame(manager: &mut EntityManager) {
    let id = manager.create_entity();
    manager.components.sprites.insert(
        id,
        true,
        Sprite {
            visible: true,
            imgid: Some(IMGID_FRAME),
            translation: Vector {
                x: 0.0,
                y: 0.0,
                z: Z_FRAME,
            },
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
