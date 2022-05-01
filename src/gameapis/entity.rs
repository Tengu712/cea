pub mod bg;
pub mod bullet;
pub mod bullet_enemy;
pub mod constant;
pub mod enemy;
pub mod frame;
pub mod player;
pub mod player_slow;
pub mod script_1_1;
pub mod text;

pub use bg::*;
pub use bullet::*;
pub use bullet_enemy::*;
pub use constant::*;
pub use enemy::*;
pub use frame::*;
pub use player::*;
pub use player_slow::*;
pub use script_1_1::*;
pub use text::*;

use super::asset::*;
use super::component::*;
use super::*;

#[derive(Default)]
pub struct EntityManager {
    next_entity_id: EntityID,
    pub components: Components,
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
