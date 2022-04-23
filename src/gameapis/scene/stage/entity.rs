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

pub struct ResultEntityUpdate {
    pub score_add: u64,
    pub gameover: bool,
    pub gameclear: bool,
}

pub struct Entity {
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
    ) -> (Self, LinkedList<Request>, ResultEntityUpdate) {
        // Start
        let mut reqs = LinkedList::new();
        let mut self_mut = self;
        self_mut.cnt_phs += is_shooting as u32;
        // Update enemy
        self_mut.enemy = self_mut.enemy.update(cnt_all as f32);
        reqs.append(&mut self_mut.enemy.create_reqs_body());
        // Update player
        self_mut.player = self_mut.player.update(PLAYER_RECT, inp);
        reqs.append(&mut self_mut.player.create_reqs_body());
        // Launch bullet
        if is_shooting {
            if stage == 1 {
                self_mut.bullets.append(&mut create_stage1_bullet(
                    &self_mut.player,
                    &self_mut.enemy,
                    self_mut.phase,
                    self_mut.cnt_phs,
                ));
            }
        }
        // Update bullet and check hit
        let mut flg_hit = 0;
        let mut flg_graze = 0;
        self_mut.bullets = self_mut
            .bullets
            .into_iter()
            .filter_map(|i| {
                i.update(BULLET_RECT)
                    .map(|n| {
                        if self_mut.player.check_hit(n.pos, n.r[0]) {
                            flg_hit += 1;
                            None
                        } else {
                            flg_graze += self_mut.player.check_hit(n.pos, n.r[1]) as u32;
                            reqs.append(&mut n.create_reqs());
                            Some(n)
                        }
                    })
                    .unwrap_or(None)
            })
            .collect();
        // Calculate
        self_mut.graze += flg_graze;
        let score_add = flg_graze as u64 * 10;
        let gameover = flg_hit > 0;
        let gameclear = false;
        // UI
        reqs.append(&mut self_mut.enemy.create_reqs_hp_gage());
        reqs.push_back(
            TextDesc::new()
                .set_text(self_mut.graze)
                .set_rect(GRAZE_RECT)
                .pack(),
        );
        // Finish
        (
            self_mut,
            reqs,
            ResultEntityUpdate {
                score_add,
                gameover,
                gameclear,
            },
        )
    }
}
