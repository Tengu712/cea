use super::*;

pub const UNIQUE_CAMERA: EntityKey = "Camera";
pub const UNIQUE_CAMERA_LEAN: EntityKey = "CameraLean";
pub const UNIQUE_ENEMY: EntityKey = "Enemy";
pub const UNIQUE_PLAYER: EntityKey = "Player";
pub const UNIQUE_PLAYER_RATE: EntityKey = "PlayerRate";

pub const UNIQUE_LOGUE1: EntityKey = "Logue1";
pub const UNIQUE_STAGE1: EntityKey = "Stage1";

pub const MESSAGE_PLAYER_HIT: MessageKey = "PlayerHit";
pub const MESSAGE_PLAYER_HIT_FRAGILE: MessageKey = "PlayerHitFragile";
pub const MESSAGE_PLAYER_GRAZE: MessageKey = "PlayerGraze";
pub const MESSAGE_ENEMY_HIT: MessageKey = "EnemyHit";
pub const MESSAGE_BONUS: MessageKey = "Bonus";

pub const TEAM_ENEMY_BULLET: usize = 1;
pub const TEAM_ENEMY_BULLET_FRAGILE: usize = 2;
pub const TEAM_ENEMY_BULLET_GRAZE: usize = 3;
pub const TEAM_PLAYER_BULLET: usize = 4;
pub const TEAM_BONUS: usize = 5;
