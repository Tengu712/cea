use super::*;

impl Stage {
    pub(super) fn remove_bullets(&mut self, world: &mut World) {
        let mut remove_ids = Vec::new();
        let mut positions = Vec::new();
        for (k, s, v) in world.emngr.coms.collisions.iter() {
            if !s.is_active() {
                continue;
            }
            if v.team != TEAM_ENEMY_BULLET && v.team != TEAM_ENEMY_BULLET_FRAGILE {
                continue;
            }
            if let Some(n) = world.emngr.coms.positions.get(k) {
                remove_ids.push(*k);
                positions.push(n.clone());
            }
        }
        for i in remove_ids {
            world.emngr.remove_entity(&i);
        }
        for i in positions {
            create_bonus(&mut world.emngr, i.x, i.y);
        }
    }
}
