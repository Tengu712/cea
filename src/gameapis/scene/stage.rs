mod constant;
mod entity;
mod logue;
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
    state: State,
    logue: logue::Logue,
    entity: Entity,
}
impl Stage {
    pub fn new() -> Self {
        Self {
            stage: 1,
            cnt_all: 0,
            state: State::Start,
            logue: logue::Logue::new(),
            entity: Entity::new(),
        }
    }
    pub fn update(self, keystates: &KeyStates) -> (Scene, LinkedList<Request>) {
        let cnt_all = self.cnt_all + 1;
        // Do task that reacts with input
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
        // Update logue
        let logue = match self.state {
            State::Start | State::End => self.logue.update(inp.snap),
            _ => self.logue,
        };
        // Update state
        let state = match self.state {
            State::Start if logue.is_end_start_log(self.stage) => State::Shoot,
            State::End if logue.is_end_log(self.stage) => State::Shoot,
            _ => self.state,
        };
        // Update entity
        let is_shooting = match state {
            State::Shoot => true,
            _ => false,
        };
        let (entity, reqs_entity) = self.entity.update(is_shooting, self.stage, cnt_all, inp);
        // Finish
        let mut reqs = reqs_entity;
        match state {
            State::Start => reqs.append(&mut logue.create_reqs(self.stage)),
            _ => (),
        }
        (
            Scene::Stage(Stage {
                stage: self.stage,
                cnt_all,
                state,
                logue,
                entity,
            }),
            reqs,
        )
    }
}
