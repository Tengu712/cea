use super::*;

const FLOOR_SIZE: f32 = 350.0;

pub fn create_floor(manager: &mut EntityManager, cnt: u32) -> EntityID {
    let id = manager.create_entity();
    manager.components.sprite3ds.insert(
        id,
        Sprite {
            imgid: Some(IMGID_FLOOR),
            translation: Vector {
                x: 0.0,
                y: FLOOR_SIZE * (cnt as f32),
                z: 50.0,
            },
            scaling: Vector {
                x: FLOOR_SIZE,
                y: FLOOR_SIZE,
                z: 1.0,
            },
            color: COLOR_WHITE,
            ..Default::default()
        },
    );
    id
}
