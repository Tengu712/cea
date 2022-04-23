/// [essensial]
pub mod bullet;
/// [essensial]
/// The enemy moves according to an instruction.
pub mod enemy;
/// [essensial]
/// The player moves according to an input information.
pub mod player;

pub const SCORE_RECT: [f32; 4] = [300.0, WIDTH, 0.0, HEIGHT];
pub const GRAZE_RECT: [f32; 4] = [300.0, WIDTH, 70.0, HEIGHT];

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
        inp: PlayerInput,
    ) -> (Self, LinkedList<Request>) {
        let mut reqs = LinkedList::new();
        // Update enemy
        let enemy = self.enemy.update();
        reqs.append(&mut enemy.create_reqs_body());
        // Update player
        let player = self.player.update(inp);
        reqs.append(&mut player.create_reqs_body());
        // Update bullet and check hit
        let mut bullets = LinkedList::new();
        let mut flg_hit = 0;
        let mut flg_graze = 0;
        for i in self.bullets {
            if let Some(n) = i.update() {
                if player.check_hit(n.pos, n.r[0]) {
                    flg_hit += 1;
                } else {
                    flg_graze += player.check_hit(n.pos, n.r[1]) as u32;
                    reqs.append(&mut n.create_reqs());
                    bullets.push_back(n);
                }
            }
        }
        // Launch bullet
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
        // Calculate
        let (phase, cnt_phs) = if enemy.hp[0] == 0 {
            (self.phase + 1, 0)
        } else {
            (self.phase, self.cnt_phs)
        };
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
                phase,
                cnt_phs,
                graze,
                player,
                enemy,
                bullets,
            },
            reqs,
        )
    }
    pub fn is_game_clear(&self, stage: u32) -> bool {
        self.phase >= if stage == 1 { 3 } else { 0 }
    }
    pub fn is_game_over(&self) -> bool {
        false
    }
}
