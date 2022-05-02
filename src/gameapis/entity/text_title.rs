use super::*;

pub fn create_title_text(emngr: &mut EntityManager) {
    let id = emngr.create_entity();
    emngr.insert_scripted_id(id, type_name::<MarkerTitleText>());
    emngr.coms.counters.insert(
        id,
        Counter {
            speed: 1,
            count: 0,
            count_max: std::i64::MAX,
        },
    );
    emngr.coms.texts.insert(
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
