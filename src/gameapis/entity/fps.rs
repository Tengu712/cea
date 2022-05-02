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
