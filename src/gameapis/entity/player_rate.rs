use super::*;

pub fn create_player_rate(emngr: &mut EntityManager) -> EntityID {
    let id = emngr.create_entity();
    emngr.coms.counters.insert(
        id,
        Counter {
            speed: 0,
            count: 0,
            count_max: 1000,
        },
    );
    emngr.coms.sprites.insert(id, Sprite::default());
    emngr
        .coms
        .valuesprites
        .insert(id, ValueSprite(Some(value_sprite_gage)));
    id
}
fn value_sprite_gage(n: &Counter) -> Sprite {
    let width = (GAME_RIGHT - GAME_LEFT) * (n.count.max(0) as f32 / n.count_max.max(1) as f32);
    Sprite {
        imgid: Some(IMGID_GAGE),
        translation: Vector {
            x: GAME_LEFT + width / 2.0,
            y: GAME_BOTTOM + 10.0,
            z: Z_VALUE,
        },
        scaling: Vector {
            x: width,
            y: 20.0,
            z: 1.0,
        },
        color: COLOR_WHITE,
        ..Default::default()
    }
}
