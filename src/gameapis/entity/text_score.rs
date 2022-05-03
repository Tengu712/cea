use super::*;

pub fn create_score(emngr: &mut EntityManager, score: i64) -> EntityID {
    let id = emngr.create_entity();
    emngr.coms.counters.insert(
        id,
        Counter {
            speed: 0,
            count: score,
            count_max: score,
        },
    );
    emngr.coms.texts.insert(
        id,
        Text {
            layer: Z_VALUE,
            text: String::default(),
            rect: Rect {
                l: 280.0,
                r: SCREEN_WIDTH,
                t: 0.0,
                b: SCREEN_HEIGHT,
            },
            rgba: COLOR_WHITE,
            fontname: "Consolas\0",
            size: 60.0,
            align: TextAlign::Left,
        },
    );
    emngr.coms.valuetexts.insert(
        id,
        ValueText {
            format: Some(|n| Text {
                layer: Z_VALUE,
                text: format!("{:>012}", n.count),
                rect: Rect {
                    l: 280.0,
                    r: SCREEN_WIDTH,
                    t: 0.0,
                    b: SCREEN_HEIGHT,
                },
                rgba: COLOR_WHITE,
                fontname: "Consolas\0",
                size: 60.0,
                align: TextAlign::Left,
            }),
        },
    );
    id
}
