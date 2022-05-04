use super::*;

const COLOR: [Vector4D; 7] = [
    Vector4D {
        x: 177.0 / 255.0,
        y: 255.0 / 255.0,
        z: 255.0 / 255.0,
        w: 1.0,
    },
    Vector4D {
        x: 177.0 / 255.0,
        y: 255.0 / 255.0,
        z: 179.0 / 255.0,
        w: 1.0,
    },
    Vector4D {
        x: 252.0 / 255.0,
        y: 242.0 / 255.0,
        z: 174.0 / 255.0,
        w: 1.0,
    },
    Vector4D {
        x: 255.0 / 255.0,
        y: 196.0 / 255.0,
        z: 177.0 / 255.0,
        w: 1.0,
    },
    Vector4D {
        x: 255.0 / 255.0,
        y: 177.0 / 255.0,
        z: 254.0 / 255.0,
        w: 1.0,
    },
    Vector4D {
        x: 216.0 / 255.0,
        y: 177.0 / 255.0,
        z: 255.0 / 255.0,
        w: 1.0,
    },
    Vector4D {
        x: 177.0 / 255.0,
        y: 193.0 / 255.0,
        z: 255.0 / 255.0,
        w: 1.0,
    },
];

pub fn create_player_hp(emngr: &mut EntityManager, cnt: usize) -> EntityID {
    let id = emngr.create_entity();
    emngr.coms.sprites.insert(
        id,
        Sprite {
            imgid: Some(IMGID_LIFE),
            mode: 1.0,
            translation: Vector {
                x: GAME_LEFT + (cnt as f32) * 24.0 + 12.0,
                y: -120.0 + SCREEN_HEIGHT / 2.0,
                z: Z_VALUE,
            },
            scaling: Vector {
                x: 64.0,
                y: 30.0,
                z: 1.0,
            },
            color: COLOR[(cnt % 7) as usize].clone(),
            ..Default::default()
        },
    );
    id
}
