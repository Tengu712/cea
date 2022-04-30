use super::*;

pub fn initialize_title_scene(manager: &mut EntityManager) {
    create_title_text(manager);
    create_fps(manager);
}
