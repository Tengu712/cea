use super::*;

use super::bullet::*;
use super::constant::*;
use super::enemy::Enemy;
use super::hp::Hp;
use super::player::Player;
use super::rate::Rate;

const DELAY_HIT_FRAGILE: u32 = 10;
const SCORE_RECT: [f32; 4] = [280.0, WIDTH, 0.0, HEIGHT];
const GRAZE_RECT: [f32; 4] = [280.0, WIDTH, 60.0, HEIGHT];
const TIME_RECT: [f32; 4] = [0.0, WIDTH - 280.0, 0.0, HEIGHT];

pub(super) struct Entity {
    // Counter
    phase: usize,
    cnt_phs: u32,
    cnt_hit_fragile: u32,
    // Value
    rate: Rate,
    hp: Hp,
    zanki: i32,
    graze: u32,
    score: u64,
    // Entity
    player: Player,
    enemy: Enemy,
    e_buls: EnemyBullets,
    p_buls: PlayerBullets,
}
impl Entity {
    pub(super) fn new(stage: usize, score: u64) -> Self {
        Self {
            phase: 0,
            cnt_phs: 0,
            cnt_hit_fragile: 0,
            rate: Rate::new(),
            hp: Hp::new(stage, 0),
            zanki: 3,
            graze: 0,
            score,
            player: Player::new(),
            enemy: Enemy::new(),
            e_buls: EnemyBullets::new(),
            p_buls: PlayerBullets::new(),
        }
    }
    pub(super) fn update(
        self,
        is_shooting: bool,
        is_game_over: bool,
        stage: usize,
        inp: PlayerInput,
    ) -> Self {
        // Update enemy
        let enemy = self.enemy.update();
        // Update player
        let player = if self.cnt_hit_fragile == 0 {
            self.player.update(inp.clone())
        } else {
            self.player
        };
        // Update enemy's bullet and check hit
        let mut e_buls = EnemyBullets::new();
        let mut is_hit = false;
        let mut is_hit_fragile = false;
        let mut cnt_graze = 0;
        for i in 0..self.e_buls.len() {
            if let Some(mut n) = self.e_buls.update_nth(i) {
                if !is_game_over && check_hit(player.pos, n.pos, n.knd.r) {
                    if n.knd.is_fragile {
                        is_hit_fragile = true;
                    } else {
                        is_hit = true;
                    }
                } else {
                    if !is_game_over && !n.is_grazed && check_hit(player.pos, n.pos, n.knd.r * 3.0)
                    {
                        cnt_graze += 1;
                        n.is_grazed = true;
                    }
                    e_buls.push(n);
                }
            }
        }
        // Update player's bullet
        let mut p_buls = PlayerBullets::new();
        let mut damage_sum = 0;
        for i in 0..self.p_buls.len() {
            if let Some(n) = self.p_buls.update_nth(i) {
                if check_hit(enemy.pos, n.pos, n.knd.r) {
                    damage_sum += n.dmg;
                } else {
                    p_buls.push(n);
                }
            }
        }
        // Launch bullet
        if is_shooting {
            if !is_hit && !is_hit_fragile && self.cnt_hit_fragile == 0 {
                player.shoot(&mut p_buls);
            }
            launch_bullet(
                &mut e_buls,
                stage,
                &player,
                &enemy,
                self.phase,
                self.cnt_phs,
            );
        }
        // Calculate
        let (cnt_hit_fragile, player, rate, zanki) =
            if is_hit || self.cnt_hit_fragile >= DELAY_HIT_FRAGILE {
                (0, player.die(), self.rate.die(), self.zanki - 1)
            } else if self.cnt_hit_fragile > 0 || is_hit_fragile {
                if inp.cnt_z == 1 {
                    (
                        0,
                        player,
                        self.rate.update_while_hit_fragile(true),
                        self.zanki,
                    )
                } else {
                    (
                        self.cnt_hit_fragile + 1,
                        player,
                        self.rate.update_while_hit_fragile(false),
                        self.zanki,
                    )
                }
            } else {
                (
                    0,
                    player,
                    self.rate.update(cnt_graze, inp.cnt_z),
                    self.zanki,
                )
            };
        let hp = self.hp.update(damage_sum);
        let graze = self.graze + cnt_graze;
        let score = self.score + cnt_graze as u64 * 10;
        // Check move phase
        let time_limit = get_time_limit(stage, self.phase);
        let (hp, phase, cnt_phs) = if hp.is_dead() || self.cnt_phs > time_limit {
            (Hp::new(stage, self.phase + 1), self.phase + 1, 0)
        } else {
            (hp, self.phase, self.cnt_phs + is_shooting as u32)
        };
        // Finish
        Self {
            // Counter
            phase,
            cnt_phs,
            cnt_hit_fragile,
            // Value
            rate,
            hp,
            zanki,
            graze,
            score,
            // Entity
            player,
            enemy,
            e_buls,
            p_buls,
        }
    }
    pub(super) fn push_reqs(
        &self,
        reqs: &mut Vec<Request>,
        stage: usize,
        is_shooting: bool,
        is_game_over: bool,
    ) {
        self.enemy.push_reqs(reqs);
        if !is_game_over {
            self.player.push_body_reqs(reqs);
        }
        reqs.push(Request::Overlay);
        for i in self.e_buls.get_vec() {
            i.push_reqs(reqs);
        }
        reqs.push(Request::Multiple);
        for i in self.p_buls.get_vec() {
            i.push_reqs(reqs);
        }
        if !is_game_over {
            self.player.push_slow_reqs(reqs);
            self.rate.push_reqs(reqs, self.player.pos);
        }
        self.hp.push_reqs(reqs, self.enemy.pos);
        reqs.push(
            TextDesc::new()
                .set_text(format!("{:>012}", self.score))
                .set_rect(SCORE_RECT)
                .set_format(TextFormat::Score)
                .pack(),
        );
        reqs.push(
            TextDesc::new()
                .set_text(self.graze)
                .set_rect(GRAZE_RECT)
                .set_format(TextFormat::Graze)
                .pack(),
        );
        if is_shooting {
            let time_limit = get_time_limit(stage, self.phase);
            reqs.push(
                TextDesc::new()
                    .set_text((time_limit as i32 - self.cnt_phs as i32).max(0) / 60)
                    .set_rect(TIME_RECT)
                    .set_align(TextAlign::Right)
                    .set_format(TextFormat::Score)
                    .pack(),
            );
        }
    }
    pub(super) fn is_game_clear(&self, stage: usize) -> bool {
        is_game_clear(stage, self.phase)
    }
    pub(super) fn is_game_over(&self) -> bool {
        self.zanki == 0
    }
}
fn check_hit(pos1: [f32; 2], pos2: [f32; 2], r: f32) -> bool {
    (pos1[0] - pos2[0]).powf(2.0) + (pos1[1] - pos2[1]).powf(2.0) < r.powf(2.0)
}
