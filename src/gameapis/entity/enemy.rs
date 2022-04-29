use super::*;

pub fn create_enemy(components: &mut Components) {
    components.sprites.insert(
        components.next_entity_id,
        Sprite {
            layer: 0,
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
    components.next_entity_id += 1;
}
