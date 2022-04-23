use super::*;

pub const STAGE1_LOG_SIZE: usize = 4;
pub const STAGE1_START_LOG_SIZE: usize = 2;
pub const STAGE1_LOG: [(&str, ImgID); STAGE1_LOG_SIZE] = [
    ("はろーわーるど", ImgID::StFlan),
    ("ほげ", ImgID::StFlan),
    ("ど", ImgID::StFlan),
    ("ろ", ImgID::StFlan),
];

pub fn create_stage1_bullet(_: &Player, enemy: &Enemy, phase: u32, cnt_phs: u32) -> LinkedList<Bullet> {
    let mut bullets = LinkedList::new();
    if phase == 0 {
        if cnt_phs % 16 == 0 {
            for i in 0..32 {
                bullets.push_back(
                    Bullet::new()
                        .set_pos(enemy.pos)
                        .set_vel(8.0)
                        .set_deg(360.0 / 32.0 * i as f32),
                );
            }
        }
    }
    bullets
}
