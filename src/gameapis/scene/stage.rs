mod bullet;
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
use enemy::Enemy;
use player::Player;
use stage1::*;
use std::collections::LinkedList;

// Base
const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 960.0;
const GAME_LEFT: f32 = -320.0;
const GAME_RIGHT: f32 = 320.0;
const GAME_TOP: f32 = 480.0;
const GAME_BOTTOM: f32 = -480.0;
// Entity
const PLAYER_RECT: [f32; 4] = [
    GAME_LEFT + 50.0,
    GAME_RIGHT - 50.0,
    GAME_TOP - 200.0,
    GAME_BOTTOM + 80.0,
];
const BULLET_RECT: [f32; 4] = [
    GAME_LEFT - 80.0,
    GAME_RIGHT + 80.0,
    GAME_TOP + 80.0,
    GAME_BOTTOM - 80.0,
];
// UI
const OPTOIN_UNCURSORED: [f32; 4] = [0.5, 0.5, 0.5, 0.5];
const LOG_RECT: [f32; 4] = [340.0, 940.0, 700.0, HEIGHT];
const PAUSE_RESUME_RECT: [f32; 4] = [400.0, WIDTH, 500.0, HEIGHT];
const PAUSE_TITLE_RECT: [f32; 4] = [400.0, WIDTH, 564.0, HEIGHT];

enum State {
    Start(u32),
    Shoot,
    End(u32),
}

pub struct Stage {
    stage: u32,
    count: u32,
    pause: u32,
    score: u64,
    state: State,
    player: Player,
    enemy: Enemy,
    bullet: LinkedList<Bullet>,
}
impl Stage {
    pub fn new() -> Self {
        Self {
            stage: 1,
            count: 0,
            pause: 0,
            score: 0,
            state: State::Start(0),
            player: Player::new(),
            enemy: Enemy::new(),
            bullet: LinkedList::new(),
        }
    }
    pub fn from(prev: Self) -> Self {
        let mut res = Stage::new();
        res.stage = prev.stage + 1;
        res.score = prev.score;
        res
    }
    pub fn update(self, keystates: &KeyStates) -> (Scene, LinkedList<Request>) {
        let mut reqs = LinkedList::new();
        //   == Do task that reacts with input ==
        // Logue
        let state = match self.state {
            _ if self.pause > 0 => self.state,
            State::Start(n) if keystates.z == 1 => {
                if self.stage == 1 && n + 1 >= STAGE1_START_LOG_SIZE {
                    State::Shoot
                } else {
                    State::Start(n + 1)
                }
            }
            State::End(n) if keystates.z == 1 => {
                if n + 1 >= STAGE1_START_LOG_SIZE {
                    State::Shoot
                } else {
                    State::End(n + 1)
                }
            }
            _ => self.state,
        };
        // Pause
        let pause = if keystates.e == 1 {
            2 * (self.pause == 0) as u32
        } else if self.pause > 0 && (keystates.up == 1 || keystates.down == 1) {
            self.pause + 1
        } else if self.pause > 0 && keystates.z == 1 {
            if self.pause % 2 == 0 {
                0
            } else {
                return (Scene::Title(Title::new()), reqs);
            }
        } else {
            self.pause
        };
        // Player input
        let (inp, slow) = if pause > 0 {
            (self.player.inp, self.player.slow)
        } else {
            let inp_x = (keystates.right > 0) as i32 - (keystates.left > 0) as i32;
            let inp_y = (keystates.up > 0) as i32 - (keystates.down > 0) as i32;
            ([inp_x, inp_y], keystates.s > 0)
        };
        //   == Calculate information ==
        // Update data
        let (count, score, player, enemy, bullet) = if pause > 0 {
            (self.count, self.score, self.player, self.enemy, self.bullet)
        } else {
            let count = self.count + 1 - pause as u32;
            let player = self.player.update(PLAYER_RECT, inp, slow);
            let enemy = self.enemy.update(count as f32);
            let bullet = self
                .bullet
                .into_iter()
                .filter_map(|n| n.update(BULLET_RECT))
                .collect::<LinkedList<Bullet>>();
            (count, self.score, player, enemy, bullet)
        };
        //   == Build request list ==
        // Entities
        reqs.append(&mut enemy.create_requests());
        reqs.append(&mut player.create_requests());
        for i in bullet.iter() {
            reqs.append(&mut i.create_requests());
        }
        // Score
        reqs.push_back(
            TextDesc::new()
                .set_text(format!("{:>012}", score))
                .set_rect([300.0, 1280.0, 0.0, 960.0])
                .set_format(TextFormat::Score)
                .pack(),
        );
        // Logue
        let log = match state {
            State::Start(n) if self.stage == 1 => Some(get_stage1_start_log(n)),
            State::End(n) if self.stage == 1 => Some(get_stage1_start_log(n)),
            _ => None,
        };
        if let Some(n) = log {
            reqs.push_back(
                TextDesc::new()
                    .set_text(n)
                    .set_rect(LOG_RECT)
                    .set_format(TextFormat::Normal)
                    .pack(),
            );
        }
        // Pause
        if pause > 0 {
            reqs.push_back(
                TextDesc::new()
                    .set_text("RESUME")
                    .set_rect(PAUSE_RESUME_RECT)
                    .set_rgba(if pause % 2 == 0 {
                        [1.0; 4]
                    } else {
                        OPTOIN_UNCURSORED
                    })
                    .set_format(TextFormat::Option)
                    .pack(),
            );
            reqs.push_back(
                TextDesc::new()
                    .set_text("GO TO TITLE")
                    .set_rect(PAUSE_TITLE_RECT)
                    .set_rgba(if pause % 2 == 1 {
                        [1.0; 4]
                    } else {
                        OPTOIN_UNCURSORED
                    })
                    .set_format(TextFormat::Option)
                    .pack(),
            );
        }
        (
            Scene::Stage(Self {
                stage: self.stage,
                count,
                score,
                pause,
                state,
                player,
                enemy,
                bullet,
            }),
            reqs,
        )
    }
}
