pub mod bullet_enemy;
pub mod bullet_player;
pub mod constant;
pub mod enemy;
pub mod floor;
pub mod fps;
pub mod frame;
pub mod player;
pub mod player_slow;
pub mod stage1;
pub mod script_camera;
pub mod script_camera_lean;
pub mod text_graze;
pub mod text_score;
pub mod text_title;

pub use bullet_enemy::*;
pub use bullet_player::*;
pub use constant::*;
pub use enemy::*;
pub use floor::*;
pub use fps::*;
pub use frame::*;
pub use player::*;
pub use player_slow::*;
pub use stage1::*;
pub use script_camera::*;
pub use script_camera_lean::*;
pub use text_graze::*;
pub use text_score::*;
pub use text_title::*;

use super::asset::*;
use super::component::*;
use super::*;

#[derive(Default)]
pub struct EntityManager {
    next_entity_id: EntityID,
    pub components: Components,
    pub unique_ids: HashMap<EntityKey, EntityID>,
    pub bullet_ids: HashSet<EntityID>,
    pub messages: HashMap<MessageKey, usize>,
    pub scripted_ids: HashMap<ScriptKey, HashSet<EntityID>>,
    pub input: Input,
    pub camera: Camera,
}
impl EntityManager {
    pub fn create_entity(&mut self) -> EntityID {
        self.next_entity_id += 1;
        self.next_entity_id - 1
    }
    pub fn remove_entity(&mut self, id: &EntityID) {
        for (_, v) in self.scripted_ids.iter_mut() {
            v.remove(id);
        }
        self.unique_ids.retain(|_, v| v != id);
        self.bullet_ids.remove(id);
        self.components.remove(id);
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
