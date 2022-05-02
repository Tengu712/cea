use super::*;

pub enum BulletKind {
    BigCircle,
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
        }
    }
}

pub fn create_bullet(
    manager: &mut EntityManager,
    knd: BulletKind,
    x: f32,
    y: f32,
    deg: f32,
    speed: f32,
    color: Vector4D,
    is_fragile: bool,
) {
    if manager.bullet_ids.len() >= BULLET_MAX_NUM {
        return;
    }
    let (size, r, imgid) = knd.analyze(is_fragile);
    // Normal
    let id = manager.create_entity();
    manager.components.velocities.insert(
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
    manager
        .components
        .positions
        .insert(id, Vector { x, y, z: Z_BULLET });
    manager
        .components
        .removerects
        .insert(id, BULLET_REMOVE_RECT);
    manager.components.collisions.insert(
        id,
        Collision {
            r,
            team: TEAM_ENEMY_BULLET,
        },
    );
    manager.components.sprites.insert(
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
    manager.bullet_ids.insert(id);
    // Graze
    let id_graze = manager.create_entity();
    manager
        .components
        .positions
        .insert(id_graze, Vector { x, y, z: Z_BULLET });
    manager
        .components
        .sameposition2ds
        .insert(id_graze, SamePosition2D(id));
    manager.components.collisions.insert(
        id_graze,
        Collision {
            r: r * 4.0,
            team: TEAM_ENEMY_BULLET_GRAZE,
        },
    );
}
