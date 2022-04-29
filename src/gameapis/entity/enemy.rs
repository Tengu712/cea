use super::*;

pub fn create_enemy(manager: &mut EntityManager) {
    let id = manager.create_entity(None);
    manager.components.sprites.insert(
        id,
        Sprite {
            visible: true,
            layer: LAYER_ENEMY,
            imgid: None,
            scaling: Vector {
                x: 140.0,
                y: 140.0,
                z: 1.0,
            },
            color: COLOR_WHITE,
            ..Default::default()
        },
    );
}
