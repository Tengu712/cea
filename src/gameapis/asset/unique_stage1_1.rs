use super::*;

pub fn unique_stage1_1(emngr: &mut EntityManager) {
    let counter = match emngr.unique_ids.get(UNIQUE_STAGE1) {
        Some(id) => match emngr.coms.counters.get(id) {
            Some(n) => n.clone(),
            None => return,
        },
        None => return,
    };
    if counter.count % 10 != 0 {
        return;
    }
    let e_pos = match emngr.unique_ids.get(UNIQUE_ENEMY) {
        Some(id) => match emngr.coms.positions.get(id) {
            Some(e_pos) => e_pos.clone(),
            None => return,
        },
        None => return,
    };
    // big circle
    for i in 0..6 {
        let is_fragile = if (counter.count / 10) % 2 == 0 {
            i % 2 == 1
        } else {
            i % 2 == 0
        };
        let _ = create_bullet(
            emngr,
            BulletKind::BigCircle,
            e_pos.x,
            e_pos.y,
            i as f32 * 60.0 + 45.0 - counter.count as f32,
            8.0,
            Vector4D {
                x: 1.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
            is_fragile,
        );
    }
    /*
    for i in 0..8 {
        let knd = if (cnt_phs / 10) % 2 == 0 {
            if i % 2 == 0 {
                BUL_MID_CIRCLE
            } else {
                BUL_MID_CIRCLE_FRAGILE
            }
        } else {
            if i % 2 == 0 {
                BUL_MID_CIRCLE_FRAGILE
            } else {
                BUL_MID_CIRCLE
            }
        };
        e_buls.push(
            Bullet::new(knd)
                .set_pos(enemy.pos)
                .set_deg(i as f32 * 60.0 - cnt_phs as f32)
                .set_col([0.0, 0.0, 1.0, 1.0])
                .set_vel(6.0),
        );
    }
    for i in 0..5 {
        e_buls.push(
            Bullet::new(BUL_CIRCLE)
                .set_pos(enemy.pos)
                .set_deg(-3.0 * (2 + i) as f32 + cnt_phs as f32 * 3.6)
                .set_col([0.0, 0.0, 1.0, 1.0])
                .set_vel(5.0),
        );
    }
    */
}
