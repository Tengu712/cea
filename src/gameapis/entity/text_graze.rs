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
                l: 280.0,
                r: SCREEN_WIDTH,
                t: 50.0,
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
