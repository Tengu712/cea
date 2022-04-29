use super::*;

pub fn create_player(components: &mut Components) {
    components
        .playeranimations
        .insert(components.next_entity_id, PlayerAnimation::default());
    components
        .playerinputs
        .insert(components.next_entity_id, PlayerInput);
    components.positions.insert(
        components.next_entity_id,
        Position {
            x: 0.0,
            y: -280.0,
            z: 0.0,
        },
    );
    components.restricts.insert(
        components.next_entity_id,
        Rect3D {
            l: GAME_LEFT + 10.0,
            r: GAME_RIGHT - 10.0,
            t: GAME_TOP - 150.0,
            b: GAME_BOTTOM + 20.0,
            n: 0.0,
            f: 0.0,
        },
    );
    components.sprites.insert(
        components.next_entity_id,
        Sprite {
            layer: 1,
            imgid: Some(IMGID_FLAN_B0),
            scaling: Vector {
                x: 100.0,
                y: 100.0,
                z: 1.0,
            },
            color: COLOR_WHITE,
            ..Default::default()
        },
    );
    components.velocities.insert(
        components.next_entity_id,
        Velocity {
            direction: Vector::default(),
            speed: 8.0,
        },
    );
    components.next_entity_id += 1;
}
