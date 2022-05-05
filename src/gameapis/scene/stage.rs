mod add_rate;
mod check_hit;
mod logue;
pub mod new;
mod remove_bullets;
mod player;
pub mod update;

pub use new::*;
pub use update::*;

use super::*;

#[derive(Default)]
pub struct Stage {
    // player
    player: EntityID,
    player_slow1: EntityID,
    player_slow2: EntityID,
    player_hitcircle: EntityID,
    rate: EntityID,
    p_hp: Vec<EntityID>,
    // hp
    e_hp: EntityID,
    // delay
    snap_delay: EntityID,
    rate_delay: EntityID,
    // value
    score: EntityID,
    graze: EntityID,
    // logue
    logue: EntityID,
    logue_left: EntityID,
    logue_right: EntityID,
    // Other
    camera_lean: EntityID,
    stage: EntityID,
    gameover: EntityID,
}
