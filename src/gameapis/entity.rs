use super::asset::*;
use super::component::*;

pub fn create_player(components: &mut Components) {
    components
        .velocities
        .insert(components.next_entity_id, Velocity::default());
    components
        .positions
        .insert(components.next_entity_id, Position::default());
    components.sprites.insert(
        components.next_entity_id,
        Sprite {
            layer: 0,
            imgid: Some(IMGID_FLAN_B0),
            scaling: Vector {
                x: 100.0,
                y: 100.0,
                z: 100.0,
            },
            color: Vector4D {
                x: 1.0,
                y: 1.0,
                z: 1.0,
                w: 1.0,
            },
            ..Default::default()
        },
    );
    components.next_entity_id += 1;
}
