use super::*;

pub fn create_stage1(manager: &mut EntityManager) -> EntityID {
    let id = manager.create_entity();
    manager.components.counters.insert(
        id,
        Counter {
            speed: 1,
            count: 0,
            count_max: 3600,
        },
    );
    manager.components.texts.insert(
        id,
        Text {
            text: String::from("0"),
            rect: Rect {
                l: 0.0,
                r: SCREEN_WIDTH - 280.0,
                t: 0.0,
                b: SCREEN_HEIGHT,
            },
            size: 60.0,
            rgba: COLOR_WHITE,
            fontname: "Consolas\0",
            align: TextAlign::Right,
        },
    );
    id
}
