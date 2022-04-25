/// [essensial]
pub mod bullet;
/// [essensial]
/// The enemy moves according to an instruction.
pub mod enemy;
/// [essensial]
/// The player moves according to an input information.
pub mod player;

const SCORE_RECT: [f32; 4] = [280.0, WIDTH, 0.0, HEIGHT];
const GRAZE_RECT: [f32; 4] = [280.0, WIDTH, 60.0, HEIGHT];
const TIME_RECT: [f32; 4] = [0.0, WIDTH - 280.0, 0.0, HEIGHT];

use super::*;
use bullet::*;
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
    pbullets: LinkedList<Bullet>,
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
            pbullets: LinkedList::new(),
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
        let mut enemy = self.enemy.update();
        reqs.append(&mut enemy.create_reqs_body());
        // Update player
        let player = self.player.update(inp);
        reqs.append(&mut player.create_reqs_body());
        // Update enemy's bullet and check hit
        let mut bullets = LinkedList::new();
        let mut flg_hit = 0;
        let mut flg_graze = 0;
        for i in self.bullets {
            if let Some(n) = i.update() {
                println!("{:p}", &n);
                if check_hit(player.pos, n.pos, n.knd.r) {
                    flg_hit += 1;
                } else {
                    flg_graze += check_hit(player.pos, n.pos, n.knd.r * 3.0) as u32;
                    reqs.append(&mut n.create_reqs());
                    bullets.push_back(n);
                }
            }
        }
        // Update player's bullet
        let mut pbullets = LinkedList::new();
        let mut damage_sum = 0;
        for i in self.pbullets {
            if let Some(n) = i.update() {
                if check_hit(enemy.pos, n.pos, n.knd.r) {
                    damage_sum += n.dmg;
                } else {
                    reqs.append(&mut n.create_reqs());
                    pbullets.push_back(n);
                }
            }
        }
        // Launch bullet
        if is_shooting {
            pbullets.append(&mut player.shoot());
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
        enemy.hp[0] = enemy.hp[0] - damage_sum;
        let (phase, cnt_phs) = if enemy.hp[0] <= 0 {
            (self.phase + 1, 0)
        } else {
            (self.phase, self.cnt_phs + is_shooting as u32)
        };
        let graze = flg_graze;
        let score = flg_graze as u64 * 10;
        // UI
        reqs.append(&mut enemy.create_reqs_hp_gage());
        if is_shooting {
            reqs.push_back(
                TextDesc::new()
                    .set_text(get_time_count(stage, phase, cnt_phs))
                    .set_rect(TIME_RECT)
                    .set_align(TextAlign::Right)
                    .set_format(TextFormat::Score)
                    .pack(),
            );
        }
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
                .set_format(TextFormat::Graze)
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
                pbullets,
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
fn get_time_count(stage: u32, phase: u32, cnt_phs: u32) -> u32 {
    let time_limit = if stage == 1 && phase < STAGE1_PHASE_SIZE as u32 {
        STAGE1_TIMELIMIT[phase as usize]
    } else {
        1
    };
    (time_limit as i32 - cnt_phs as i32).max(0) as u32 / 60
}
fn check_hit(pos1: [f32; 2], pos2: [f32; 2], r: f32) -> bool {
    if (pos1[0] - pos2[0]).powf(2.0) + (pos1[1] - pos2[1]).powf(2.0) < r.powf(2.0) {
        true
    } else {
        false
    }
}
