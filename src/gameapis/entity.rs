pub mod enemy;
pub mod fps;
pub mod frame;
pub mod player;
pub mod playerslow;

pub use enemy::*;
pub use fps::*;
pub use frame::*;
pub use player::*;
pub use playerslow::*;

use super::asset::*;
use super::component::*;
use super::*;

const SCREEN_WIDTH: f32 = 1280.0;
const SCREEN_HEIGHT: f32 = 960.0;
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

const LAYER_ENEMY: u32 = 1;
const LAYER_PLAYER: u32 = 2;
const LAYER_PLAYER_SLOW: u32 = 3;
const LAYER_FRAME: u32 = 4;
const LAYER_FPS: u32 = 5;

#[derive(Default)]
pub struct EntityManager {
    pub next_entity_id: EntityID,
    pub entities: Entities,
    pub components: Components,
    pub input: Input,
}
impl EntityManager {
    pub fn create_entity(&mut self, key: Option<EntityKey>) -> EntityID {
        let res = self.next_entity_id;
        self.next_entity_id += 1;
        if let Some(n) = key {
            self.entities.insert(n, res);
        }
        res
    }
}