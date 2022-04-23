/// [essensial]
pub mod bullet;
/// [essensial]
/// The enemy moves according to an instruction.
pub mod enemy;
/// [essensial]
/// The player moves according to an input information.
pub mod player;

use super::*;
use bullet::Bullet;
use enemy::Enemy;
use player::{Player, PlayerInput};

pub struct Entity {
    score: u64,
    phase: u32,
    cnt_phs: u32,
    graze: u32,
    player: Player,
    enemy: Enemy,
    bullets: LinkedList<Bullet>,
}
impl Entity {
    pub fn new() -> Self {
        Self {
            score: 0,
            phase: 0,
            cnt_phs: 0,
            graze: 0,
            player: Player::new(),
            enemy: Enemy::new(),
            bullets: LinkedList::new(),
        }
    }
    pub fn update(
        self,
        is_shooting: bool,
        stage: u32,
        cnt_all: u32,
        inp: PlayerInput,
    ) -> (Self, LinkedList<Request>) {
        let mut reqs = LinkedList::new();
        // Update enemy
        let enemy = self.enemy.update(cnt_all as f32);
        reqs.append(&mut enemy.create_reqs_body());
        // Update player
        let player = self.player.update(PLAYER_RECT, inp);
        reqs.append(&mut player.create_reqs_body());
        // Launch bullet
        let mut bullets = LinkedList::new();
        if is_shooting {
            if stage == 1 {
                bullets.append(&mut create_stage1_bullet(
                    &player,
                    &enemy,
                    self.phase,
                    self.cnt_phs,
                ));
            }
        }
        // Update bullet and check hit
        let mut flg_hit = 0;
        let mut flg_graze = 0;
        for i in self.bullets {
            if let Some(n) = i.update(BULLET_RECT) {
                if player.check_hit(n.pos, n.r[0]) {
                    flg_hit += 1;
                } else {
                    flg_graze += player.check_hit(n.pos, n.r[1]) as u32;
                    reqs.append(&mut n.create_reqs());
                    bullets.push_back(n);
                }
            }
        }
        // Calculate
        let graze = flg_graze;
        let score = flg_graze as u64 * 10;
        // UI
        reqs.append(&mut enemy.create_reqs_hp_gage());
        reqs.push_back(
            TextDesc::new()
                .set_text(format!("{:>012}", self.score))
                .set_rect(SCORE_RECT)
                .set_format(TextFormat::Score)
                .pack(),
        );
        reqs.push_back(
            TextDesc::new()
                .set_text(self.graze)
                .set_rect(GRAZE_RECT)
                .pack(),
        );
        // Finish
        (
            Self {
                score,
                phase: self.phase,
                cnt_phs: self.cnt_phs,
                graze,
                player,
                enemy,
                bullets,
            },
            reqs,
        )
    }
}
