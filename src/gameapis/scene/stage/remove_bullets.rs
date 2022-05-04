use super::*;

impl Stage {
    pub(super) fn remove_bullets(&mut self, emngr: &mut EntityManager) {
        let mut remove_ids = Vec::new();
        let mut positions = Vec::new();
        for (k, s, v) in emngr.coms.collisions.iter() {
            if !s.is_active() {
                continue;
            }
            if v.team != TEAM_ENEMY_BULLET && v.team != TEAM_ENEMY_BULLET_FRAGILE {
                continue;
            }
            if let Some(n) = emngr.coms.positions.get(k) {
                remove_ids.push(*k);
                positions.push(n.clone());
            }
        }
        for i in remove_ids {
            emngr.remove_entity(&i);
        }
        for i in positions {
            create_bonus(emngr, i.x, i.y);
        }
    }
}
