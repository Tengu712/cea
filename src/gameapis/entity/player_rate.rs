use super::*;

pub fn create_player_rate(emngr: &mut EntityManager) -> EntityID {
    let id = emngr.create_entity();
    emngr.insert_scripted_id(id, type_name::<MarkerGage>());
    emngr.coms.counters.insert(
        id,
        Counter {
            speed: 0,
            count: 0,
            count_max: 1000,
        },
    );
    emngr.coms.sprites.insert(id, Sprite::default());
    emngr.coms.valuesprites.insert(
        id,
        ValueSprite {
            format: Some(|n| Sprite {
                imgid: None,
                translation: Vector {
                    x: 0.0,
                    y: GAME_BOTTOM,
                    z: Z_VALUE,
                },
                scaling: Vector {
                    x: (GAME_RIGHT - GAME_RIGHT)
                        * (n.count.max(0) as f32 / n.count_max.max(1) as f32),
                    y: 20.0,
                    z: 1.0,
                },
                color: COLOR_WHITE,
                ..Default::default()
            }),
        },
    );
    id
}
