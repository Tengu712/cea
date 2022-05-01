use super::*;

pub fn create_fps(manager: &mut EntityManager) {
    let id = manager.create_entity();
    manager
        .components
        .fpsmeasures
        .insert(id, FpsMeasure::default());
    manager.components.texts.insert(
        id,
        Text {
            visible: true,
            text: String::new(),
            rect: Rect {
                l: 0.0,
                r: SCREEN_WIDTH - 10.0,
                t: SCREEN_HEIGHT - 40.0,
                b: SCREEN_HEIGHT,
            },
            rgba: COLOR_WHITE,
            fontname: "Consolas\0",
            size: 32.0,
            align: TextAlign::Right,
        },
    );
}

pub fn create_title_text(manager: &mut EntityManager) {
    let id = manager.create_entity();
    manager.insert_scripted_id(id, type_name::<MarkerTitleText>());
    manager.components.counters.insert(
        id,
        Counter {
            count: 0,
            count_max: 0xffffffff,
        },
    );
    manager.components.texts.insert(
        id,
        Text {
            visible: true,
            text: String::from("PRESS ANY KEY TO START"),
            rect: Rect {
                l: 0.0,
                r: SCREEN_WIDTH,
                t: 720.0,
                b: SCREEN_HEIGHT,
            },
            rgba: COLOR_WHITE,
            fontname: "游明朝\0",
            size: 56.0,
            align: TextAlign::Center,
        },
    );
}
