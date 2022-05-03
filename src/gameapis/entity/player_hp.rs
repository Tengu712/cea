use super::*;

pub fn create_player_hp(emngr: &mut EntityManager, cnt: usize) -> EntityID {
    let id = emngr.create_entity();
    emngr.coms.sprites.insert(
        id,
        Sprite {
            imgid: None,
            translation: Vector {
                x: 290.0 + (cnt as f32) * 30.0 - SCREEN_WIDTH / 2.0,
                y: -110.0 + SCREEN_HEIGHT / 2.0,
                z: Z_VALUE,
            },
            scaling: Vector {
                x: 30.0,
                y: 30.0,
                z: 1.0,
            },
            color: COLOR_WHITE,
            ..Default::default()
        },
    );
    id
}
