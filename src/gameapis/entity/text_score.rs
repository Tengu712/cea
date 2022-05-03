use super::*;

pub fn create_score(emngr: &mut EntityManager, score: i64) -> EntityID {
    let id = emngr.create_entity();
    emngr.coms.texts.insert(id, Text::default());
    emngr.coms.counters.insert(
        id,
        Counter {
            speed: 0,
            count: score,
            count_max: score,
        },
    );
    emngr.coms.valuetexts.insert(
        id,
        ValueText(Some(|n, _| Text {
            layer: Z_VALUE,
            text: format!("{:>012}", n.count),
            rect: Rect {
                l: SCREEN_WIDTH / 2.0 + GAME_LEFT,
                r: SCREEN_WIDTH,
                t: 12.0,
                b: SCREEN_HEIGHT,
            },
            rgba: COLOR_WHITE,
            fontname: "Consolas\0",
            size: 60.0,
            align: TextAlign::Left,
        })),
    );
    id
}
