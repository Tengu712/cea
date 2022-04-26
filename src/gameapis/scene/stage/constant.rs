use super::*;

use super::bullet::*;
use super::enemy::Enemy;
use super::player::Player;

const STAGE_SIZE: usize = 3;

const START_LOG_SIZE: [usize; STAGE_SIZE] = [2, 0, 0];
pub(super) fn is_end_start_log(stage: usize, cnt: usize) -> bool {
    stage >= STAGE_SIZE || cnt >= START_LOG_SIZE[stage]
}

const END_LOG_SIZE: [usize; STAGE_SIZE] = [4, 0, 0];
pub(super) fn is_end_log(stage: usize, cnt: usize) -> bool {
    stage >= STAGE_SIZE || cnt >= END_LOG_SIZE[stage]
}

const LOG_MAX_SIZE: usize = 4;
const DEFAULT_LOG: (&'static str, Option<ImgID>, bool) = ("", None, false);
const LOG: [[(&'static str, Option<ImgID>, bool); LOG_MAX_SIZE]; STAGE_SIZE] = [
    [
        ("はろーわーるど", Some(IMGID_FLAN_ST0), false),
        ("ほげ", Some(IMGID_FLAN_ST0), true),
        DEFAULT_LOG,
        DEFAULT_LOG,
    ],
    [DEFAULT_LOG, DEFAULT_LOG, DEFAULT_LOG, DEFAULT_LOG],
    [DEFAULT_LOG, DEFAULT_LOG, DEFAULT_LOG, DEFAULT_LOG],
];
pub(super) fn get_log(stage: usize, cnt: usize) -> (&'static str, Option<ImgID>, bool) {
    if stage >= STAGE_SIZE || cnt >= END_LOG_SIZE[stage] || cnt >= LOG_MAX_SIZE {
        DEFAULT_LOG
    } else {
        LOG[stage][cnt].clone()
    }
}

const PHASE_SIZE: [usize; STAGE_SIZE] = [3, 4, 5];
pub(super) fn is_game_clear(stage: usize, phase: usize) -> bool {
    stage >= STAGE_SIZE || phase >= PHASE_SIZE[stage]
}

const PHASE_MAX_SIZE: usize = 5;
const TIMELIMIT: [[u32; PHASE_MAX_SIZE]; STAGE_SIZE] = [
    [3600, 3600, 3600, 0, 0],
    [3600, 3600, 3600, 0, 0],
    [3600, 3600, 3600, 0, 0],
];
pub(super) fn get_time_limit(stage: usize, phase: usize) -> u32 {
    if stage >= STAGE_SIZE || phase >= PHASE_SIZE[stage] || phase >= PHASE_MAX_SIZE {
        0
    } else {
        TIMELIMIT[stage][phase]
    }
}
const MAXHP: [[i32; PHASE_MAX_SIZE]; STAGE_SIZE] = [
    [2000, 2000, 10000, 0, 0],
    [10000, 10000, 10000, 0, 0],
    [10000, 10000, 10000, 0, 0],
];
pub(super) fn get_max_hp(stage: usize, phase: usize) -> i32 {
    if stage >= STAGE_SIZE || phase >= PHASE_SIZE[stage] || phase >= PHASE_MAX_SIZE {
        0
    } else {
        MAXHP[stage][phase]
    }
}

fn create_stage1_bullet(
    _: &Player,
    enemy: &Enemy,
    phase: usize,
    cnt_phs: u32,
) -> LinkedList<Bullet> {
    let mut bullets = LinkedList::new();
    if phase == 0 {
        if cnt_phs % 8 == 0 {
            bullets.push_back(Bullet::new(BUL_CIRCLE).set_pos(enemy.pos).set_vel(5.0));
        }
    }
    bullets
}
pub(super) fn launch_bullet(
    stage: usize,
    player: &Player,
    enemy: &Enemy,
    phase: usize,
    cnt_phs: u32,
) -> LinkedList<Bullet> {
    if stage == 0 {
        create_stage1_bullet(player, enemy, phase, cnt_phs)
    } else {
        LinkedList::new()
    }
}
