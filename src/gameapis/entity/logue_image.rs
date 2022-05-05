use super::*;

pub fn create_logue_image(emngr: &mut EntityManager) -> EntityID {
    let id = emngr.create_entity();
    emngr.coms.sprites.insert(id, Sprite::default());
    id
}
