use super::*;

pub fn system_remove_rect(emngr: &mut EntityManager) {
    let mut deads = Vec::new();
    for (k, s, v) in emngr.coms.removerects.iter() {
        if !s.is_active() {
            continue;
        }
        if let Some(n) = emngr.coms.positions.get(&k) {
            if n.x < v.l || n.x > v.r || n.y < v.b || n.y > v.t || n.z < v.n || n.z > v.f {
                deads.push(*k);
            }
        }
    }
    for i in deads {
        emngr.remove_entity(&i);
    }
}
