pub mod enemy;
pub mod player;

pub use enemy::*;
pub use player::*;

use super::asset::*;
use super::component::*;

//const WIDTH: f32 = 1280.0;
//const HEIGHT: f32 = 960.0;
const GAME_LEFT: f32 = -392.0;
const GAME_RIGHT: f32 = 392.0;
const GAME_TOP: f32 = 480.0;
const GAME_BOTTOM: f32 = -480.0;
const COLOR_WHITE: Vector4D = Vector4D {
    x: 1.0,
    y: 1.0,
    z: 1.0,
    w: 1.0,
};




pub fn create_text(components: &mut Components) {
    components.texts.insert(
        components.next_entity_id,
        Text {
            layer: 0,
            text: String::from("はろーわーるど"),
            rect: Rect {
                l: 0.0,
                r: 1280.0,
                t: 0.0,
                b: 960.0,
            },
            rgba: COLOR_WHITE,
            fontname: "游明朝\0",
            size: 64.0,
            align: TextAlign2::Left,
        },
    );
    components.next_entity_id += 1;
}
