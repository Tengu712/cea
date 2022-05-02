use super::*;

pub fn create_enemy(manager: &mut EntityManager) -> EntityID {
    let id = manager.create_entity();
    manager.components.positions.insert(
        id,
        Vector {
            x: 0.0,
            y: 280.0,
            z: Z_ENEMY,
        },
    );
    manager.components.sprites.insert(
        id,
        Sprite {
            imgid: Some(IMGID_REMILIA_F0),
            scaling: Vector {
                x: 140.0,
                y: 140.0,
                z: 1.0,
            },
            color: COLOR_WHITE,
            ..Default::default()
        },
    );
    id
}
