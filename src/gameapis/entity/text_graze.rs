use super::*;

pub fn create_graze(manager: &mut EntityManager, graze: u64) -> EntityID {
    let id = manager.create_entity();
    manager.components.counters.insert(
        id,
        Counter {
            speed: 0,
            count: graze,
            count_max: graze,
        },
    );
    manager.components.texts.insert(
        id,
        Text {
            visible: true,
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
    manager
        .components
        .valuetexts
        .insert(id, ValueText::default());
    id
}
