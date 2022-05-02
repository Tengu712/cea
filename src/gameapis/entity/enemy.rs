use super::*;

pub fn create_enemy(emngr: &mut EntityManager) -> EntityID {
    let id = emngr.create_entity();
    emngr.coms.positions.insert(
        id,
        Vector {
            x: 0.0,
            y: 280.0,
            z: Z_ENEMY,
        },
    );
    emngr.coms.sprites.insert(
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
