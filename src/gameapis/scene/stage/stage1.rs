use super::*;

pub const STAGE1_LOG_SIZE: usize = 4;
pub const STAGE1_START_LOG_SIZE: usize = 2;
pub const STAGE1_LOG: [&str; STAGE1_LOG_SIZE] = [
    "はろーわーるど",
    "ほげ",
    "ほげ",
    "ほげ",
];

pub fn create_stage1_bullet(cnt_chp: u32, _: &Player, enemy: &Enemy) -> LinkedList<Bullet> {
    let mut bullets = LinkedList::new();
    if enemy.phase == 0 {
        if cnt_chp % 16 == 0 {
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
