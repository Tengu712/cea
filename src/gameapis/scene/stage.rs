mod bullet;
mod constant;
mod enemy;
mod player;
mod stage1;

use super::{
    super::{
        input::KeyStates,
        request::{cdata::*, text::*, *},
    },
    title::Title,
    Scene,
};
use bullet::Bullet;
use constant::*;
use enemy::Enemy;
use player::Player;
use stage1::*;
use std::collections::LinkedList;

enum State {
    Start,
    Shoot,
    End,
}

pub struct Stage {
    stage: u32,
    cnt_all: u32,
    cnt_chp: u32,
    cnt_log: usize,
    graze: u32,
    score: u64,
    state: State,
    player: Player,
    enemy: Enemy,
    bullets: LinkedList<Bullet>,
}
impl Stage {
    pub fn new() -> Self {
        Self {
            stage: 1,
            cnt_all: 0,
            cnt_log: 0,
            cnt_chp: 0,
            graze: 0,
            score: 0,
            state: State::Start,
            player: Player::new(),
            enemy: Enemy::new(),
            bullets: LinkedList::new(),
        }
    }
    pub fn from(stage: u32, score: u64) -> Self {
        let mut res = Stage::new();
        res.stage = stage;
        res.score = score;
        res
    }
    pub fn update(self, keystates: &KeyStates) -> (Scene, LinkedList<Request>) {
        let cnt_all = self.cnt_all + 1;
        // ======================================================================================== //
        //   Do task that reacts with input
        // ======================================================================================== //
        // Log count
        let (state, cnt_log) = match self.state {
            State::Shoot => (self.state, self.cnt_log),
            _ if keystates.z != 1 => (self.state, self.cnt_log),
            State::Start if self.cnt_log + 1 >= get_startlogsize(self.stage) => {
                (State::Shoot, self.cnt_log)
            }
            State::End if self.cnt_log + 1 >= get_logsize(self.stage) => {
                if self.stage == 3 {
                    return (Scene::Title(Title::new()), LinkedList::new());
                } else {
                    return (
                        Scene::Stage(Stage::from(self.stage + 1, self.score)),
                        LinkedList::new(),
                    );
                }
            }
            _ => (self.state, self.cnt_log + 1),
        };
        // Player input
        let (inp, slow, snap, atack) = {
            let inp_x = (keystates.right > 0) as i32 - (keystates.left > 0) as i32;
            let inp_y = (keystates.up > 0) as i32 - (keystates.down > 0) as i32;
            ([inp_x, inp_y], keystates.s > 0, keystates.z == 1, keystates.x == 1)
        };
        // ======================================================================================== //
        //   Update entity
        // ======================================================================================== //
        let mut reqs_entity = LinkedList::new();
        // Update counter
        let mut cnt_chp = match state {
            State::Shoot => self.cnt_chp + 1,
            _ => 0,
        };
        let mut score = self.score;
        let mut graze = self.graze;
        // Update entities
        let (enemy, mut reqs_enemy) = self.enemy.update(cnt_all as f32);
        reqs_entity.append(&mut reqs_enemy);
        let (player, mut reqs_player) = self.player.update(PLAYER_RECT, inp, slow);
        reqs_entity.append(&mut reqs_player);
        let mut bullets = LinkedList::new();
        match state {
            State::Shoot if self.stage == 1 => {
                bullets.append(&mut create_stage1_bullet(cnt_chp, &player, &enemy, 0))
            }
            _ => (),
        }
        for i in self.bullets {
            let bullet_n = i.update(BULLET_RECT);
            if let Some((n, mut reqs)) = bullet_n {
                let (is_hit, is_graze) = player.check_hit(n.pos, n.r);
                if is_graze {
                    score += 10; // !SCORE!
                    graze += 1; // !GRAZE!
                }
                if !is_hit {
                    bullets.push_back(n);
                    reqs_entity.append(&mut reqs);
                }
            }
        }
        // ======================================================================================== //
        //   Build ui requests
        // ======================================================================================== //
        let mut reqs_ui = LinkedList::new();
        reqs_ui.append(&mut enemy.create_reqs_hp_gage());
        reqs_ui.push_back(
            TextDesc::new()
                .set_text(format!("{:>012}", score))
                .set_rect(SCORE_RECT)
                .set_format(TextFormat::Score)
                .pack(),
        );
        match state {
            State::Shoot => (),
            _ if self.stage == 1 => {
                let n = STAGE1_LOG[cnt_log];
                reqs_ui.push_back(
                    TextDesc::new()
                        .set_text(n)
                        .set_rect(LOG_RECT)
                        .set_format(TextFormat::Normal)
                        .pack(),
                );
            }
            _ => (),
        };
        // Finish
        let mut reqs = reqs_entity;
        reqs.append(&mut reqs_ui);
        (
            Scene::Stage(Stage {
                stage: self.stage,
                cnt_all,
                cnt_chp,
                cnt_log,
                graze,
                score,
                state,
                player,
                enemy,
                bullets,
            }),
            reqs,
        )
    }
}
fn get_startlogsize(stage: u32) -> usize {
    if stage == 1 {
        STAGE1_START_LOG_SIZE
    } else {
        0
    }
}
fn get_logsize(stage: u32) -> usize {
    if stage == 1 {
        STAGE1_LOG_SIZE
    } else {
        0
    }
}
