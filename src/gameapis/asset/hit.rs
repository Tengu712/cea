use super::*;

pub(super) fn hit(
    emngr: &mut EntityManager,
    p_id: &EntityID,
    b_team: usize,
    message: MessageKey,
    is_remove: bool,
) {
    let mut remove_ids = Vec::new();
    if let Some(p_pos) = emngr.coms.positions.get(p_id) {
        for (b_id, s, b_cll) in emngr.coms.collisions.iter() {
            if !s.is_active() {
                continue;
            }
            if let Some(b_pos) = emngr.coms.positions.get(b_id) {
                if b_cll.team != b_team {
                    continue;
                }
                if check_hit([p_pos.x, p_pos.y], [b_pos.x, b_pos.y], b_cll.r) {
                    match emngr.messages.get_mut(message) {
                        Some(n) => *n += 1,
                        None => {
                            emngr.messages.insert(message, 1);
                        }
                    }
                    if is_remove {
                        remove_ids.push(*b_id);
                    }
                }
            }
        }
    }
    for i in remove_ids {
        emngr.remove_entity(&i);
    }
}
fn check_hit(pos1: [f32; 2], pos2: [f32; 2], r: f32) -> bool {
    (pos1[0] - pos2[0]).powf(2.0) + (pos1[1] - pos2[1]).powf(2.0) < r.powf(2.0)
}
