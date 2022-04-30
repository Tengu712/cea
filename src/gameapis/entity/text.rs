use super::*;

pub fn create_fps(manager: &mut EntityManager) {
    let id = manager.create_entity(None);
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
    let id = manager.create_entity(None);
    manager.components.textanimators.insert(
        id,
        TextAnimator {
            is_active: true,
            count: 0,
            f: title_text_animation,
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
