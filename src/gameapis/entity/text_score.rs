use super::*;

pub fn create_score(manager: &mut EntityManager, score: u64) -> EntityID {
    let id = manager.create_entity();
    manager.components.counters.insert(
        id,
        Counter {
            speed: 0,
            count: score,
            count_max: std::u64::MAX,
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
                t: 0.0,
                b: SCREEN_HEIGHT,
            },
            rgba: COLOR_WHITE,
            fontname: "Consolas\0",
            size: 60.0,
            align: TextAlign::Left,
        },
    );
    manager.components.valuetexts.insert(
        id,
        ValueText {
            format: Some(|n| format!("{:>012}", n)),
        },
    );
    id
}
