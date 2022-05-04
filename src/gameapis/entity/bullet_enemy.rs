use super::*;

/*
pub(super) const BUL_CIRCLE: BulletKind = BulletKind {
    imgid: IMGID_BUL_CIRCLE,
    size: 30.0,
    r: 10.0,
    is_fragile: false,
};
pub(super) const BUL_CIRCLE_FRAGILE: BulletKind = BulletKind {
    imgid: IMGID_BUL_CIRCLE_FRAGILE,
    size: 30.0,
    r: 10.0,
    is_fragile: true,
};
*/

pub enum BulletKind {
    BigCircle,
    MidCircle,
    Circle,
}
impl BulletKind {
    fn analyze(&self, is_fragile: bool) -> (f32, f32, &'static str) {
        match self {
            BulletKind::BigCircle => (
                110.0,
                20.0,
                if is_fragile {
                    IMGID_BUL_BIG_CIRCLE_FRAGILE
                } else {
                    IMGID_BUL_BIG_CIRCLE
                },
            ),
            BulletKind::MidCircle => (
                50.0,
                20.0,
                if is_fragile {
                    IMGID_BUL_CIRCLE_FRAGILE
                } else {
                    IMGID_BUL_CIRCLE
                },
            ),
            BulletKind::Circle => (
                30.0,
                10.0,
                if is_fragile {
                    IMGID_BUL_CIRCLE_FRAGILE
                } else {
                    IMGID_BUL_CIRCLE
                },
            ),
        }
    }
}

pub fn create_bullet(
    emngr: &mut EntityManager,
    knd: BulletKind,
    x: f32,
    y: f32,
    deg: f32,
    speed: f32,
    color: Vector4D,
    is_fragile: bool,
) {
    if emngr.bullet_ids.len() >= BULLET_MAX_NUM {
        return;
    }
    let (size, r, imgid) = knd.analyze(is_fragile);
    // Normal
    let id = emngr.create_entity();
    emngr.coms.velocities.insert(
        id,
        Velocity {
            direction: Vector {
                x: deg.to_radians().cos(),
                y: deg.to_radians().sin(),
                z: 0.0,
            },
            speed,
        },
    );
    emngr
        .coms
        .positions
        .insert(id, Vector { x, y, z: Z_BULLET });
    emngr
        .coms
        .removerects
        .insert(id, BULLET_REMOVE_RECT);
    emngr.coms.collisions.insert(
        id,
        Collision {
            r,
            team: if is_fragile {
                TEAM_ENEMY_BULLET_FRAGILE
            } else {
                TEAM_ENEMY_BULLET
            },
        },
    );
    emngr.coms.sprites.insert(
        id,
        Sprite {
            imgid: Some(imgid),
            scaling: Vector {
                x: size,
                y: size,
                z: 1.0,
            },
            color,
            mode: 1.0,
            ..Default::default()
        },
    );
    emngr.bullet_ids.insert(id);
    // Graze
    let id_graze = emngr.create_entity();
    emngr
        .coms
        .positions
        .insert(id_graze, Vector { x, y, z: Z_BULLET });
    emngr
        .coms
        .sameposition2ds
        .insert(id_graze, SamePosition2D(id));
    emngr.coms.collisions.insert(
        id_graze,
        Collision {
            r: r * 4.0,
            team: TEAM_ENEMY_BULLET_GRAZE,
        },
    );
}
