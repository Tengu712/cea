use super::*;

pub fn create_graze(emngr: &mut EntityManager, graze: i64) -> EntityID {
    let id = emngr.create_entity();
    emngr.coms.counters.insert(
        id,
        Counter {
            speed: 0,
            count: graze,
            count_max: graze,
        },
    );
    emngr.coms.texts.insert(
        id,
        Text {
            layer: Z_VALUE,
            text: String::default(),
            rect: Rect {
                l: SCREEN_WIDTH / 2.0 + GAME_LEFT,
                r: SCREEN_WIDTH,
                t: 55.0,
                b: SCREEN_HEIGHT,
            },
            rgba: COLOR_WHITE,
            fontname: "Consolas\0",
            size: 56.0,
            align: TextAlign::Left,
        },
    );
    emngr
        .coms
        .valuetexts
        .insert(id, ValueText::default());
    id
}
