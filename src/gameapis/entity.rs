pub mod enemy;
pub mod frame;
pub mod player;
pub mod text;

pub use enemy::*;
pub use frame::*;
pub use player::*;
pub use text::*;

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

const Z_ENEMY: f32 = -4.0;
const Z_PLAYER: f32 = -3.0;
const Z_PLAYER_SLOW: f32 = -2.0;
const Z_FRAME: f32 = -1.0;

#[derive(Default)]
pub struct EntityManager {
    pub next_entity_id: EntityID,
    pub components: Components,
    pub scripted_ids: HashMap<ScriptKey, HashSet<EntityID>>,
    pub input: Input,
}
impl EntityManager {
    pub fn create_entity(&mut self) -> EntityID {
        self.next_entity_id += 1;
        self.next_entity_id - 1
    }
    pub fn insert_scripted_id(&mut self, id: EntityID, key: ScriptKey) {
        if let Some(ids) = self.scripted_ids.get_mut(key) {
            ids.insert(id);
        } else {
            let mut ids = HashSet::new();
            ids.insert(id);
            self.scripted_ids.insert(key, ids);
        }
    }
}
