use super::*;

pub fn system_remove_rect(manager: &mut EntityManager) {
    let mut deads = Vec::new();
    for (k, s, v) in manager.components.removerects.iter() {
        if !s.is_active() {
            continue;
        }
        if let Some(n) = manager.components.positions.get(&k) {
            if n.x < v.l || n.x > v.r || n.y < v.b || n.y > v.t || n.z < v.n || n.z > v.f {
                deads.push(*k);
            }
        }
    }
    for i in deads {
        manager.remove_entity(&i);
    }
}
