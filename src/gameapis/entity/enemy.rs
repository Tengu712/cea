use super::*;

pub fn create_enemy(manager: &mut EntityManager) -> EntityID {
    let id = manager.create_entity();
    manager.components.sprites.insert(
        id,
        true,
        Sprite {
            visible: true,
            imgid: None,
            translation: Vector {
                x: 0.0,
                y: 0.0,
                z: Z_ENEMY,
            },
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
