use super::*;

pub fn create_frame(emngr: &mut EntityManager) -> EntityID {
    let id = emngr.create_entity();
    emngr.coms.sprites.insert(
        id,
        Sprite {
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
    id
}
