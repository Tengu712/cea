use super::entity::bullet::*;
use super::*;

pub const STAGE1_LOG_SIZE: usize = 4;
pub const STAGE1_START_LOG_SIZE: usize = 2;
pub const STAGE1_LOG: [(&str, ImgID, bool); STAGE1_LOG_SIZE] = [
    ("はろーわーるど", IMGID_FLAN_ST0, false),
    ("ほげ", IMGID_FLAN_ST0, true),
    ("ど", IMGID_FLAN_ST0, false),
    ("ろ", IMGID_FLAN_ST0, false),
];
pub const STAGE1_PHASE_SIZE: usize = 3;
pub const STAGE1_TIMELIMIT: [u32; STAGE1_PHASE_SIZE] = [3600, 3600, 3600];

pub fn create_stage1_bullet(
    _: &Player,
    enemy: &Enemy,
    phase: u32,
    cnt_phs: u32,
) -> LinkedList<Bullet> {
    let mut bullets = LinkedList::new();
    if phase == 0 {}
    bullets
}
