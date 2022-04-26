use super::*;

use super::entity::{bullet::*, enemy::*, player::*};

pub(super) const STAGE_SIZE: usize = 3;

pub(super) const START_LOG_SIZE: [usize; STAGE_SIZE] = [2, 0, 0];
pub(super) const END_LOG_SIZE: [usize; STAGE_SIZE] = [4, 0, 0];
pub(super) const LOG_MAX_SIZE: usize = 4;
pub(super) const LOG: [[(&str, Option<ImgID>, bool); LOG_MAX_SIZE]; STAGE_SIZE] = [
    [
        ("はろーわーるど", Some(IMGID_FLAN_ST0), false),
        ("ほげ", Some(IMGID_FLAN_ST0), true),
        ("", None, false),
        ("", None, false),
    ],
    [
        ("", None, false),
        ("", None, false),
        ("", None, false),
        ("", None, false),
    ],
    [
        ("", None, false),
        ("", None, false),
        ("", None, false),
        ("", None, false),
    ],
];

pub(super) const PHASE_SIZE: [usize; STAGE_SIZE] = [3, 4, 5];
pub(super) const PHASE_MAX_SIZE: usize = 5;
pub(super) const TIMELIMIT: [[u32; PHASE_MAX_SIZE]; STAGE_SIZE] = [
    [3600, 3600, 3600, 0, 0],
    [3600, 3600, 3600, 0, 0],
    [3600, 3600, 3600, 0, 0],
];
pub(super) const MAXHP: [[i32; PHASE_MAX_SIZE]; STAGE_SIZE] = [
    [10000, 10000, 10000, 0, 0],
    [10000, 10000, 10000, 0, 0],
    [10000, 10000, 10000, 0, 0],
];

pub(super) fn create_stage1_bullet(
    _: &Player,
    enemy: &Enemy,
    phase: usize,
    cnt_phs: u32,
) -> LinkedList<Bullet> {
    let mut bullets = LinkedList::new();
    if phase == 0 {}
    bullets
}
