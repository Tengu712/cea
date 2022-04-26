mod bullet;
mod constant;
mod enemy;
mod entity;
mod hp;
mod logue;
mod player;
mod rate;

use super::*;

const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 960.0;
const GAME_LEFT: f32 = -392.0;
const GAME_RIGHT: f32 = 392.0;
const GAME_TOP: f32 = 480.0;
const GAME_BOTTOM: f32 = -480.0;

struct PlayerInput {
    lr_ud: [i32; 2],
    cnt_s: i16,
    cnt_z: i16,
    cnt_x: i16,
}

enum State {
    Start,
    Shoot,
    End,
    GameOver,
}

pub(in super::super) struct Stage {
    cnt: u32,
    stage: usize,
    state: State,
    logue: logue::Logue,
    entity: entity::Entity,
}
impl Stage {
    pub(super) fn new() -> Self {
        Self {
            cnt: 0,
            stage: 0,
            state: State::Start,
            logue: logue::Logue::new(),
            entity: entity::Entity::new(0, 0),
        }
    }
    pub(super) fn update(self, keystates: &KeyStates) -> (Scene, LinkedList<Request>) {
        let cnt = self.cnt + 1;
        // Do task that reacts with input
        let inp = {
            let inp_x = (keystates.right > 0) as i32 - (keystates.left > 0) as i32;
            let inp_y = (keystates.up > 0) as i32 - (keystates.down > 0) as i32;
            PlayerInput {
                lr_ud: [inp_x, inp_y],
                cnt_s: keystates.s,
                cnt_z: keystates.z,
                cnt_x: keystates.x,
            }
        };
        // Update logue
        let logue = match self.state {
            State::Start | State::End => self.logue.update(inp.cnt_z),
            _ => self.logue,
        };
        let state = match self.state {
            State::Start if logue.is_end_start_log(self.stage) => State::Shoot,
            State::End if logue.is_end_log(self.stage) => {
                return (Scene::Stage(Stage::new()), LinkedList::new());
            }
            _ => self.state,
        };
        // Update entity
        let is_shooting = match state {
            State::Shoot => true,
            _ => false,
        };
        let (entity, reqs_entity) = self.entity.update(is_shooting, self.stage, inp);
        let state = if is_shooting && entity.is_game_over() {
            State::GameOver
        } else if is_shooting && entity.is_game_clear(self.stage) {
            State::End
        } else {
            state
        };
        // Finish
        let mut reqs = reqs_entity;
        reqs.push_back(IMGID_FRAME.pack());
        reqs.push_back(CDataDiff::new().set_scl([WIDTH, HEIGHT]).pack());
        reqs.push_back(Request::DrawImage);
        match state {
            State::Start | State::End => reqs.append(&mut logue.create_reqs(self.stage)),
            _ => (),
        }
        (
            Scene::Stage(Stage {
                cnt,
                stage: self.stage,
                state,
                logue,
                entity,
            }),
            reqs,
        )
    }
}
