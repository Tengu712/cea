use super::*;

pub fn create_fps(components: &mut Components) {
    components.fpsmeasure = (FpsMeasure::default(), components.next_entity_id);
    components.texts.insert(
        components.next_entity_id,
        Text {
            layer: LAYER_FPS,
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
            align: TextAlign2::Right,
        },
    );
    components.next_entity_id += 1;
}
