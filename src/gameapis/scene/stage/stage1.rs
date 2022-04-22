use super::*;

pub const STAGE1_START_LOG_SIZE: u32 = 2;

pub fn get_stage1_start_log(cnt: u32) -> String {
    if cnt == 0 {
        String::from("はろーわーるど")
    } else {
        String::from("ほげ")
    }
}

pub fn create_stage1_bullet(count: u32, _: &Player, enemy: &Enemy) -> LinkedList<Bullet> {
    let mut bullets = LinkedList::new();
    if enemy.phase == 0 {
        if count % 16 == 0 {
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
