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
        stage: usize,
        inp: PlayerInput,
    ) -> (Self, LinkedList<Request>) {
        let mut reqs = LinkedList::new();
        // Update enemy
        let enemy = self.enemy.update();
        reqs.append(&mut enemy.create_body_reqs());
        // Update player
        let player = if self.cnt_hit_fragile == 0 {
            self.player.update(inp.clone())
        } else {
            self.player
        };
        reqs.append(&mut player.create_body_reqs());
        // Update enemy's bullet and check hit
        reqs.push_back(Request::Overlay);
        let mut e_buls = EnemyBullets::new();
        let mut is_hit = false;
        let mut is_hit_fragile = false;
        let mut cnt_graze = 0;
        for i in 0..ENEMY_BULLETS_SIZE {
            if let Some(mut n) = self.e_buls.update_nth(i) {
                if check_hit(player.pos, n.pos, n.knd.r) {
                    if n.knd.is_fragile {
                        is_hit_fragile = true;
                    } else {
                        is_hit = true;
                    }
                } else {
                    if !n.is_grazed && check_hit(player.pos, n.pos, n.knd.r * 3.0) {
                        cnt_graze += 1;
                        n.is_grazed = true;
                    }
                    reqs.append(&mut n.create_reqs());
                    e_buls.push(n);
                }
            } else {
                println!("{}", i);
                break;
            }
        }
        reqs.push_back(Request::Multiple);
        // Update player's bullet
        let mut p_buls = PlayerBullets::new();
        let mut damage_sum = 0;
        for i in 0..PLAYER_BULLETS_SIZE {
            if let Some(n) = self.p_buls.update_nth(i) {
                if check_hit(enemy.pos, n.pos, n.knd.r) {
                    damage_sum += n.dmg;
                } else {
                    reqs.append(&mut n.create_reqs());
                    p_buls.push(n);
                }
            } else {
                break;
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
        let (cnt_hit_fragile, player, rate) = if is_hit || self.cnt_hit_fragile >= DELAY_HIT_FRAGILE
        {
            (0, player.die(), self.rate.die())
        } else if self.cnt_hit_fragile > 0 || is_hit_fragile {
            if inp.cnt_z == 1 {
                (0, player, self.rate.update_while_hit_fragile(true))
            } else {
                (
                    self.cnt_hit_fragile + 1,
                    player,
                    self.rate.update_while_hit_fragile(false),
                )
            }
        } else {
            (0, player, self.rate.update(cnt_graze, inp.cnt_z))
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
        // UI
        reqs.append(&mut player.create_slow_requests());
        reqs.append(&mut rate.create_reqs(player.pos));
        reqs.append(&mut hp.create_reqs(enemy.pos));
        if is_shooting {
            reqs.push_back(
                TextDesc::new()
                    .set_text((time_limit as i32 - cnt_phs as i32).max(0) / 60)
                    .set_rect(TIME_RECT)
                    .set_align(TextAlign::Right)
                    .set_format(TextFormat::Score)
                    .pack(),
            );
        }
        reqs.push_back(
            TextDesc::new()
                .set_text(format!("{:>012}", score))
                .set_rect(SCORE_RECT)
                .set_format(TextFormat::Score)
                .pack(),
        );
        reqs.push_back(
            TextDesc::new()
                .set_text(graze)
                .set_rect(GRAZE_RECT)
                .set_format(TextFormat::Graze)
                .pack(),
        );
        // Finish
        (
            Self {
                // Counter
                phase,
                cnt_phs,
                cnt_hit_fragile,
                // Value
                rate,
                hp,
                graze,
                score,
                // Entity
                player,
                enemy,
                e_buls,
                p_buls,
            },
            reqs,
        )
    }
    pub(super) fn is_game_clear(&self, stage: usize) -> bool {
        is_game_clear(stage, self.phase)
    }
    pub(super) fn is_game_over(&self) -> bool {
        false
    }
}
fn check_hit(pos1: [f32; 2], pos2: [f32; 2], r: f32) -> bool {
    (pos1[0] - pos2[0]).powf(2.0) + (pos1[1] - pos2[1]).powf(2.0) < r.powf(2.0)
}
