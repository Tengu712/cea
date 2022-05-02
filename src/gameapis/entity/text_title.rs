use super::*;

pub fn create_title_text(manager: &mut EntityManager) {
    let id = manager.create_entity();
    manager.insert_scripted_id(id, type_name::<MarkerTitleText>());
    manager.components.counters.insert(
        id,
        Counter {
            speed: 1,
            count: 0,
            count_max: std::i64::MAX,
        },
    );
    manager.components.texts.insert(
        id,
        Text {
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
