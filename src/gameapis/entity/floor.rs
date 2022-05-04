use super::*;

pub fn create_floor(emngr: &mut EntityManager, cnt: u32) -> EntityID {
    let id = emngr.create_entity();
    emngr.coms.sprite3ds.insert(
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
