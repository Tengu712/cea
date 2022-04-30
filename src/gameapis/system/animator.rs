use super::*;

pub fn system_update_sprite_animator(manager: &mut EntityManager) {
    for (k, v) in manager.components.spriteanimators.iter_mut() {
        if let Some(n) = manager.components.sprites.get_mut(k) {
            let f = v.f;
            f(v, n);
        }
    }
}

pub fn system_update_text_animator(manager: &mut EntityManager) {
    for (k, v) in manager.components.textanimators.iter_mut() {
        if let Some(n) = manager.components.texts.get_mut(k) {
            let f = v.f;
            f(v, n);
        }
    }
}
