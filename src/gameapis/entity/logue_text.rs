use super::*;

// The logue entity is proceeded by unique_logue.

pub fn create_logue_text(emngr: &mut EntityManager) -> EntityID {
    let id = emngr.create_entity();
    emngr.coms.counters.insert(
        id,
        Counter {
            speed: 0,
            count: 0,
            count_max: 0,
        },
    );
    emngr.coms.texts.insert(
        id,
        Text {
            layer: Z_LOGUE,
            text: String::from(""),
            rect: Rect {
                l: 340.0,
                r: 940.0,
                t: 735.0,
                b: SCREEN_HEIGHT,
            },
            fontname: "游明朝\0",
            size: 40.0,
            rgba: COLOR_WHITE,
            align: TextAlign::Left,
        },
    );
    emngr.coms.sprites.insert(
        id,
        Sprite {
            imgid: None,
            translation: Vector {
                x: 0.0,
                y: -300.0,
                z: Z_LOGUE_BOX,
            },
            scaling: Vector {
                x: (GAME_RIGHT - GAME_LEFT) as f32 * 0.85,
                y: 120.0,
                z: 1.0,
            },
            color: Vector4D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.8,
            },
            ..Default::default()
        },
    );
    id
}
