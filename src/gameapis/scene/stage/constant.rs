// Base
pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 960.0;
pub const GAME_LEFT: f32 = -320.0;
pub const GAME_RIGHT: f32 = 320.0;
pub const GAME_TOP: f32 = 480.0;
pub const GAME_BOTTOM: f32 = -480.0;
// Entity
pub const PLAYER_RECT: [f32; 4] = [
    GAME_LEFT + 50.0,
    GAME_RIGHT - 50.0,
    GAME_TOP - 200.0,
    GAME_BOTTOM + 80.0,
];
pub const BULLET_RECT: [f32; 4] = [
    GAME_LEFT - 80.0,
    GAME_RIGHT + 80.0,
    GAME_TOP + 80.0,
    GAME_BOTTOM - 80.0,
];
// UI
pub const SCORE_RECT: [f32; 4] = [300.0, WIDTH, 0.0, HEIGHT];
pub const DAMAGE_RECT: [f32; 4] = [0.0, WIDTH, 70.0, HEIGHT];
pub const LOG_RECT: [f32; 4] = [340.0, 940.0, 700.0, HEIGHT];
