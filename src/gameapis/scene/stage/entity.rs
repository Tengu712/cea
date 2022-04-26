/// [essensial]
pub(super) mod bullet;
/// [essensial]
/// The enemy moves according to an instruction.
pub(super) mod enemy;
/// [essensial]
/// The player moves according to an input information.
pub(super) mod player;

use super::*;

use super::stage1::*;
use bullet::*;
use enemy::Enemy;
use player::Player;

const SCORE_RECT: [f32; 4] = [280.0, WIDTH, 0.0, HEIGHT];
const GRAZE_RECT: [f32; 4] = [280.0, WIDTH, 60.0, HEIGHT];
const TIME_RECT: [f32; 4] = [0.0, WIDTH - 280.0, 0.0, HEIGHT];

pub(super) struct Entity {
    score: u64,
    phase: usize,
    cnt_phs: u32,
    graze: u32,
    player: Player,
    enemy: Enemy,
    bullets: LinkedList<Bullet>,
    pbullets: LinkedList<Bullet>,
}
impl Entity {
    pub(super) fn new(stage: usize) -> Self {
        Self {
            score: 0,
            phase: 0,
            cnt_phs: 0,
            graze: 0,
            player: Player::new(),
            enemy: Enemy::new(get_max_hp(stage, 0)),
            bullets: LinkedList::new(),
            pbullets: LinkedList::new(),
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
        let mut enemy = self.enemy.update();
        reqs.append(&mut enemy.create_body_reqs());
        // Update player
        let player = self.player.update(inp);
        reqs.append(&mut player.create_body_reqs());
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
        // Slowcircle
        reqs.append(&mut player.create_slow_requests());
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
        let time_limit = get_time_limit(stage, self.phase);
        let (phase, cnt_phs) = if enemy.hp[0] <= 0 || self.cnt_phs > time_limit {
            enemy.hp[0] = get_max_hp(stage, self.phase + 1);
            enemy.hp[1] = get_max_hp(stage, self.phase + 1);
            (self.phase + 1, 0)
        } else {
            (self.phase, self.cnt_phs + is_shooting as u32)
        };
        let graze = self.graze + flg_graze;
        let score = self.score + flg_graze as u64 * 10;
        // UI
        reqs.append(&mut enemy.create_reqs_hp_gage());
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
    pub(super) fn is_game_clear(&self, stage: usize) -> bool {
        stage >= STAGE_SIZE || self.phase >= PHASE_SIZE[stage]
    }
    pub(super) fn is_game_over(&self) -> bool {
        false
    }
}
fn get_time_limit(stage: usize, phase: usize) -> u32 {
    if stage >= STAGE_SIZE || phase >= PHASE_SIZE[stage] {
        1
    } else {
        TIMELIMIT[stage][phase]
    }
}
fn get_max_hp(stage: usize, phase: usize) -> i32 {
    if stage >= STAGE_SIZE || phase >= PHASE_SIZE[stage] {
        0
    } else {
        MAXHP[stage][phase]
    }
}
fn check_hit(pos1: [f32; 2], pos2: [f32; 2], r: f32) -> bool {
    if (pos1[0] - pos2[0]).powf(2.0) + (pos1[1] - pos2[1]).powf(2.0) < r.powf(2.0) {
        true
    } else {
        false
    }
}
