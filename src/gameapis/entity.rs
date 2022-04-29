use super::asset::*;
use super::component::*;

//const WIDTH: f32 = 1280.0;
//const HEIGHT: f32 = 960.0;
const GAME_LEFT: f32 = -392.0;
const GAME_RIGHT: f32 = 392.0;
const GAME_TOP: f32 = 480.0;
const GAME_BOTTOM: f32 = -480.0;
const COLOR_WHITE: Vector4D = Vector4D {
    x: 1.0,
    y: 1.0,
    z: 1.0,
    w: 1.0,
};

pub fn create_player(components: &mut Components) {
    components
        .playerinputs
        .insert(components.next_entity_id, PlayerInput);
    components.velocities.insert(
        components.next_entity_id,
        Velocity {
            direction: Vector::default(),
            speed: 8.0,
        },
    );
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
    components.next_entity_id += 1;
}

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

pub fn create_text(components: &mut Components) {
    components.texts.insert(
        components.next_entity_id,
        Text {
            layer: 0,
            text: String::from("はろーわーるど"),
            rect: Rect {
                l: 0.0,
                r: 1280.0,
                t: 0.0,
                b: 960.0,
            },
            rgba: COLOR_WHITE,
            fontname: "游明朝\0",
            size: 64.0,
            align: TextAlign2::Left,
        },
    );
    components.next_entity_id += 1;
}
