use super::*;

pub fn create_title_text(emngr: &mut EntityManager) {
    let id = emngr.create_entity();
    emngr.coms.texts.insert(id, Text::default());
    emngr.coms.counters.insert(
        id,
        Counter {
            speed: 1,
            count: 0,
            count_max: std::i64::MAX,
        },
    );
    emngr.coms.valuetexts.insert(
        id,
        ValueText(Some(|n| Text {
            layer: -1.0,
            text: String::from("PRESS ANY KEY TO START"),
            rect: Rect {
                l: 0.0,
                r: SCREEN_WIDTH,
                t: 720.0,
                b: SCREEN_HEIGHT,
            },
            rgba: Vector4D {
                x: 1.0,
                y: 1.0,
                z: 1.0,
                w: (n.count as f32).to_radians().cos().abs(),
            },
            fontname: "游明朝\0",
            size: 56.0,
            align: TextAlign::Center,
        })),
    );
}
