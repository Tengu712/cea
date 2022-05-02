use super::*;

pub fn system_restrict_position(emngr: &mut EntityManager) {
    for (k, s, v) in emngr.coms.restricts.iter() {
        if !s.is_active() {
            continue;
        }
        if let Some(n) = emngr.coms.positions.get_mut(&k) {
            n.x = n.x.max(v.l).min(v.r);
            n.y = n.y.max(v.b).min(v.t);
            n.z = n.z.max(v.n).min(v.f);
        }
    }
}
