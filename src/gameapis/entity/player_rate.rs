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
    emngr.coms.texts.insert(id, Text::default());
    emngr
        .coms
        .valuetexts
        .insert(id, ValueText(Some(value_sprite_gage)));
    id
}
fn value_sprite_gage(n: &Counter, t: &Text) -> Text {
    Text {
        layer: Z_VALUE,
        text: format!("{}%", n.count / 10),
        rect: t.rect.clone(),
        rgba: Vector4D {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 0.7,
        },
        size: 28.0,
        fontname: "consolas\0",
        align: TextAlign::Left,
    }
}
