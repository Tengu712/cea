mod bg;
mod bullet;
mod constant;
mod enemy;
mod entity;
mod gameover;
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
    bg: bg::Background,
    gameover: gameover::GameOver,
}
impl Stage {
    pub(super) fn new() -> Self {
        Self {
            cnt: 0,
            stage: 0,
            state: State::Start,
            logue: logue::Logue::new(),
            entity: entity::Entity::new(0, 0),
            bg: bg::Background::new(),
            gameover: gameover::GameOver::new(),
        }
    }
    pub(super) fn update(self, reqs: &mut Vec<Request>, input: &Input) -> Scene {
        let cnt = self.cnt + 1;
        // ========== ========== //
        let logue = match self.state {
            State::Start | State::End => self.logue.update(input.z),
            _ => self.logue,
        };
        let state = match self.state {
            State::Start if logue.is_end_start_log(self.stage) => State::Shoot,
            State::End if logue.is_end_log(self.stage) => return Scene::Stage(Stage::new()),
            _ => self.state,
        };
        let gameover = match state {
            State::GameOver if self.gameover.is_end() => {
                return Scene::Title(super::title::Title::new())
            }
            State::GameOver => self.gameover.update(),
            _ => self.gameover,
        };
        let is_shooting = match state {
            State::Shoot => true,
            _ => false,
        };
        let is_game_over = match state {
            State::GameOver => true,
            _ => false,
        };
        // 
        let entity = self.entity.update(is_shooting, is_game_over, self.stage, input);
        let state = if is_shooting && entity.is_game_over() {
            State::GameOver
        } else if is_shooting && entity.is_game_clear(self.stage) {
            State::End
        } else {
            state
        };
        // Back ground
        let bg = self.bg;
        // ========== Drawing ========== //
        bg.push_reqs(reqs, cnt);
        entity.push_reqs(reqs, self.stage, is_shooting, is_game_over);
        reqs.push(IMGID_FRAME.pack());
        reqs.push(CDataDiff::new().set_scl([WIDTH, HEIGHT]).pack());
        reqs.push(Request::DrawImage);
        match state {
            State::Start | State::End => logue.push_reqs(reqs, self.stage),
            State::GameOver => gameover.push_reqs(reqs),
            _ => (),
        }
        Scene::Stage(Stage {
            cnt,
            stage: self.stage,
            state,
            logue,
            entity,
            bg,
            gameover,
        })
    }
}
