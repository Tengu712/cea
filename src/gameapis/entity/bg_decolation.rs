use super::*;

pub fn create_bg_decolation(
    emngr: &mut EntityManager,
    cnt_y: u32,
    cnt_z: u32,
    is_right: bool,
) -> EntityID {
    let id = emngr.create_entity();
    emngr.coms.sprite3ds.insert(
        id,
        Sprite {
            imgid: Some(IMGID_DECOLATION),
            translation: Vector {
                x: 70.0 * if is_right { 1.0 } else { -1.0 },
                y: FLOOR_SIZE * (cnt_y as f32),
                z: 40.0 - 10.0 * (cnt_z as f32),
            },
            scaling: Vector {
                x: FLOOR_SIZE * 64.0 / 512.0,
                y: FLOOR_SIZE,
                z: 1.0,
            },
            color: COLOR_WHITE,
            ..Default::default()
        },
    );
    id
}
