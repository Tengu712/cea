mod constant;
mod entity;
mod stage1;

use super::{
    super::{
        input::KeyStates,
        request::{cdata::*, text::*, *},
    },
    title::Title,
    Scene,
};
use constant::*;
use entity::{
    bullet::Bullet,
    enemy::Enemy,
    player::{Player, PlayerInput},
    *,
};
use stage1::*;
use std::collections::LinkedList;

enum State {
    Start,
    Shoot,
    End,
    GameOver,
}

pub struct Stage {
    stage: u32,
    cnt_all: u32,
    cnt_log: usize,
    score: u64,
    state: State,
    entity: Entity,
}
impl Stage {
    pub fn new() -> Self {
        Self {
            stage: 1,
            cnt_all: 0,
            cnt_log: 0,
            score: 0,
            state: State::Start,
            entity: Entity::new(),
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
        // Do task that reacts with input
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
        let inp = {
            let inp_x = (keystates.right > 0) as i32 - (keystates.left > 0) as i32;
            let inp_y = (keystates.up > 0) as i32 - (keystates.down > 0) as i32;
            PlayerInput {
                lr_ud: [inp_x, inp_y],
                slow: keystates.s > 0,
                snap: keystates.z == 1,
                atack: keystates.x == 1,
            }
        };
        // Update entity
        let is_shooting = match state {
            State::Shoot => true,
            _ => false,
        };
        let (entity, reqs_entity, res) = self.entity.update(is_shooting, self.stage, cnt_all, inp);
        let state = if res.gameover {
            State::GameOver
        } else if res.gameclear {
            State::End
        } else {
            state
        };
        // UI
        let mut reqs_ui = LinkedList::new();
        reqs_ui.push_back(
            TextDesc::new()
                .set_text(format!("{:>012}", self.score))
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
                cnt_log,
                score: self.score,
                state,
                entity,
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
