use super::*;

pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 960.0;
pub const GAME_LEFT: f32 = -392.0;
pub const GAME_RIGHT: f32 = 392.0;
pub const GAME_TOP: f32 = 480.0;
pub const GAME_BOTTOM: f32 = -480.0;
pub const COLOR_WHITE: Vector4D = Vector4D {
    x: 1.0,
    y: 1.0,
    z: 1.0,
    w: 1.0,
};

pub const BULLET_REMOVE_RECT: Rect3D = Rect3D {
    l: GAME_LEFT - 80.0,
    r: GAME_RIGHT + 80.0,
    b: GAME_BOTTOM - 80.0,
    t: GAME_TOP + 80.0,
    n: -1000.0,
    f: 1000.0,
};
pub const BULLET_MAX_NUM: usize = 640;

pub const Z_ENEMY: f32 = -5.0;
pub const Z_PLAYER: f32 = -4.0;
pub const Z_BULLET: f32 = -3.0;
pub const Z_PLAYER_SLOW: f32 = -2.0;
pub const Z_FRAME: f32 = -1.0;
