use super::*;

use super::bullet::*;
use super::constant::*;
use super::enemy::Enemy;
use super::player::Player;

const SCORE_RECT: [f32; 4] = [280.0, WIDTH, 0.0, HEIGHT];
const GRAZE_RECT: [f32; 4] = [280.0, WIDTH, 60.0, HEIGHT];
const TIME_RECT: [f32; 4] = [0.0, WIDTH - 280.0, 0.0, HEIGHT];
const HP_GAGE_R: f32 = 150.0;
const HP_GAGE_SQUARE_SIZE: f32 = 4.0;

pub(super) struct Entity {
    score: u64,
    phase: usize,
    cnt_phs: u32,
    graze: u32,
    player: Player,
    enemy: Enemy,
    e_hp: [i32; 2],
    e_buls: LinkedList<Bullet>,
    p_buls: LinkedList<Bullet>,
}
impl Entity {
    pub(super) fn new(stage: usize, score: u64) -> Self {
        Self {
            score,
            phase: 0,
            cnt_phs: 0,
            graze: 0,
            player: Player::new(),
            enemy: Enemy::new(),
            e_hp: [get_max_hp(stage, 0); 2],
            e_buls: LinkedList::new(),
            p_buls: LinkedList::new(),
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
        let player = self.player.update(inp);
        reqs.append(&mut player.create_body_reqs());
        // Update enemy's bullet and check hit
        let mut e_buls = LinkedList::new();
        let mut flg_hit = 0;
        let mut flg_graze = 0;
        for i in self.e_buls {
            if let Some(n) = i.update() {
                if check_hit(player.pos, n.pos, n.knd.r) {
                    flg_hit += 1;
                } else {
                    flg_graze += check_hit(player.pos, n.pos, n.knd.r * 3.0) as u32;
                    reqs.append(&mut n.create_reqs());
                    e_buls.push_back(n);
                }
            }
        }
        // Update player's bullet
        let mut p_buls = LinkedList::new();
        let mut damage_sum = 0;
        for i in self.p_buls {
            if let Some(n) = i.update() {
                if check_hit(enemy.pos, n.pos, n.knd.r) {
                    damage_sum += n.dmg;
                } else {
                    reqs.append(&mut n.create_reqs());
                    p_buls.push_back(n);
                }
            }
        }
        // Launch bullet
        if is_shooting {
            p_buls.append(&mut player.shoot());
            e_buls.append(&mut launch_bullet(
                stage,
                &player,
                &enemy,
                self.phase,
                self.cnt_phs,
            ));
        }
        // Calculate
        let e_hp_0 = self.e_hp[0] - damage_sum;
        let time_limit = get_time_limit(stage, self.phase);
        let (e_hp, phase, cnt_phs) = if e_hp_0 <= 0 || self.cnt_phs > time_limit {
            ([get_max_hp(stage, self.phase + 1); 2], self.phase + 1, 0)
        } else {
            (
                [e_hp_0, self.e_hp[1]],
                self.phase,
                self.cnt_phs + is_shooting as u32,
            )
        };
        let graze = self.graze + flg_graze;
        let score = self.score + flg_graze as u64 * 10;
        // UI
        reqs.append(&mut create_hp_gage_reqs(e_hp, enemy.pos));
        reqs.append(&mut player.create_slow_requests());
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
                e_hp,
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
    if (pos1[0] - pos2[0]).powf(2.0) + (pos1[1] - pos2[1]).powf(2.0) < r.powf(2.0) {
        true
    } else {
        false
    }
}
fn create_hp_gage_reqs(e_hp: [i32; 2], e_pos: [f32; 2]) -> LinkedList<Request> {
    let mut reqs = LinkedList::new();
    let theta = 360.0 * e_hp[0].max(0) as f32 / e_hp[1].max(1) as f32;
    reqs.push_back(Request::UnsetImage);
    for i in 0..360 {
        if i as f32 >= theta {
            break;
        }
        reqs.push_back(
            CDataDiff::new()
                .set_trs([
                    e_pos[0] - HP_GAGE_R * (i as f32).to_radians().sin(),
                    e_pos[1] + HP_GAGE_R * (i as f32).to_radians().cos(),
                ])
                .set_rot([0.0, 0.0, (i as f32).to_radians()])
                .set_scl([HP_GAGE_SQUARE_SIZE, HP_GAGE_SQUARE_SIZE])
                .pack(),
        );
        reqs.push_back(Request::DrawImage);
    }
    reqs
}
