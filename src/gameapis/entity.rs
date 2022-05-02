pub mod bullet_enemy;
pub mod bullet_player;
pub mod camera;
pub mod camera_lean;
pub mod constant;
pub mod delay_count;
pub mod enemy;
pub mod enemy_hp;
pub mod floor;
pub mod fps;
pub mod frame;
pub mod player;
pub mod player_rate;
pub mod player_slow;
pub mod stage1;
pub mod text_graze;
pub mod text_score;
pub mod text_title;

pub use bullet_enemy::*;
pub use bullet_player::*;
pub use camera::*;
pub use camera_lean::*;
pub use constant::*;
pub use delay_count::*;
pub use enemy::*;
pub use enemy_hp::*;
pub use floor::*;
pub use fps::*;
pub use frame::*;
pub use player::*;
pub use player_rate::*;
pub use player_slow::*;
pub use stage1::*;
pub use text_graze::*;
pub use text_score::*;
pub use text_title::*;

use super::asset::*;
use super::component::*;
use super::*;

#[derive(Default)]
pub struct EntityManager {
    next_entity_id: EntityID,
    pub coms: Components,
    pub unique_ids: HashMap<EntityKey, EntityID>,
    pub bullet_ids: HashSet<EntityID>,
    pub messages: HashMap<MessageKey, i64>,
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
        self.coms.remove(id);
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
