use super::asset::*;
use super::component::*;

const COLOR_WHITE: Vector4D = Vector4D {
    x: 1.0,
    y: 1.0,
    z: 1.0,
    w: 1.0,
};

const P_SPD: f32 = 8.0;
const P_INIT_POS: Position = Position {
    x: 0.0,
    y: -280.0,
    z: 0.0,
};
const P_SCALING: Vector = Vector {
    x: 100.0,
    y: 100.0,
    z: 1.0,
};
pub fn create_player(components: &mut Components) {
    components
        .playerinputs
        .insert(components.next_entity_id, PlayerInput);
    components.velocities.insert(
        components.next_entity_id,
        Velocity {
            direction: Vector::default(),
            speed: P_SPD,
        },
    );
    components
        .positions
        .insert(components.next_entity_id, P_INIT_POS);
    components.sprites.insert(
        components.next_entity_id,
        Sprite {
            layer: 1,
            imgid: Some(IMGID_FLAN_B0),
            scaling: P_SCALING,
            color: COLOR_WHITE,
            ..Default::default()
        },
    );
    components.next_entity_id += 1;
}

pub fn create_enemy(components: &mut Components) {
    components.sprites.insert(
        components.next_entity_id,
        Sprite {
            layer: 0,
            imgid: None,
            scaling: P_SCALING,
            color: COLOR_WHITE,
            ..Default::default()
        },
    );
    components.next_entity_id += 1;
}
