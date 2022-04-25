use super::*;

use super::entity::{bullet::*, enemy::*, player::*};

pub(super) const STAGE1_LOG_SIZE: usize = 4;
pub(super) const STAGE1_START_LOG_SIZE: usize = 2;
pub(super) const STAGE1_LOG: [(&str, ImgID, bool); STAGE1_LOG_SIZE] = [
    ("はろーわーるど", IMGID_FLAN_ST0, false),
    ("ほげ", IMGID_FLAN_ST0, true),
    ("ど", IMGID_FLAN_ST0, false),
    ("ろ", IMGID_FLAN_ST0, false),
];
pub(super) const STAGE1_PHASE_SIZE: usize = 3;
pub(super) const STAGE1_TIMELIMIT: [u32; STAGE1_PHASE_SIZE] = [3600, 3600, 3600];

pub(super) fn create_stage1_bullet(
    _: &Player,
    enemy: &Enemy,
    phase: u32,
    cnt_phs: u32,
) -> LinkedList<Bullet> {
    let mut bullets = LinkedList::new();
    if phase == 0 {}
    bullets
}
