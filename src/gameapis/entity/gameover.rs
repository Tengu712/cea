use super::*;

pub fn create_gameover(emngr: &mut EntityManager) -> EntityID {
    let id = emngr.create_entity();
    emngr.coms.counters.insert(
        id,
        Counter {
            speed: 1,
            count: 0,
            count_max: 240,
        },
    );
    emngr.coms.texts.insert(
        id,
        Text {
            layer: Z_GAMEOVER,
            text: String::from("GAME OVER"),
            rect: Rect {
                l: 0.0,
                r: SCREEN_WIDTH,
                t: SCREEN_HEIGHT / 2.0 - 50.0,
                b: SCREEN_HEIGHT,
            },
            rgba: Vector4D {
                x: 1.0,
                y: 1.0,
                z: 1.0,
                w: 0.0,
            },
            size: 50.0,
            fontname: "游明朝\0",
            align: TextAlign::Center,
        },
    );
    emngr.coms.sprites.insert(
        id,
        Sprite {
            imgid: None,
            translation: Vector {
                x: 0.0,
                y: 0.0,
                z: Z_GAMEOVER,
            },
            scaling: Vector {
                x: SCREEN_WIDTH,
                y: SCREEN_HEIGHT,
                z: 1.0,
            },
            color: Vector4D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            ..Default::default()
        },
    );
    emngr.coms.valuetexts.insert(
        id,
        ValueText {
            format: Some(|n| Text {
                layer: Z_GAMEOVER,
                text: String::from("GAME OVER"),
                rect: Rect {
                    l: 0.0,
                    r: SCREEN_WIDTH,
                    t: SCREEN_HEIGHT / 2.0 - 50.0,
                    b: SCREEN_HEIGHT,
                },
                rgba: Vector4D {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                    w: ((n.count - 30) as f32 / 60.0).min(1.0).max(0.0),
                },
                size: 50.0,
                fontname: "游明朝\0",
                align: TextAlign::Center,
            }),
        },
    );
    emngr.coms.valuesprites.insert(
        id,
        ValueSprite {
            format: Some(|n| Sprite {
                imgid: None,
                translation: Vector {
                    x: 0.0,
                    y: 0.0,
                    z: Z_GAMEOVER,
                },
                scaling: Vector {
                    x: SCREEN_WIDTH,
                    y: SCREEN_HEIGHT,
                    z: 1.0,
                },
                color: Vector4D {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    w: ((n.count - 30) as f32 / 180.0).min(1.0).max(0.0),
                },
                ..Default::default()
            }),
        },
    );
    id
}
