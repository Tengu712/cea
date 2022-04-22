mod bullet;
mod constant;
mod enemy;
mod player;
mod stage1;
mod state;

use super::{
    super::{
        input::KeyStates,
        request::{cdata::*, text::*, *},
    },
    Scene,
};
use bullet::Bullet;
use constant::*;
use enemy::Enemy;
use player::Player;
use stage1::*;
use state::State;
use std::collections::LinkedList;

pub struct Stage {
    stage: u32,
    count: u32,
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
            count: 0,
            score: 0,
            state: State::Start(0),
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
        // ======================================================================================== //
        //   Do task that reacts with input
        // ======================================================================================== //
        let reqs_input = LinkedList::new();
        let (state, is_next) = self.state.update(self.stage, keystates.z == 1);
        if is_next {
            return (
                Scene::Stage(Stage::from(self.stage + 1, self.score)),
                reqs_input,
            );
        }
        let (inp, slow) = {
            let inp_x = (keystates.right > 0) as i32 - (keystates.left > 0) as i32;
            let inp_y = (keystates.up > 0) as i32 - (keystates.down > 0) as i32;
            ([inp_x, inp_y], keystates.s > 0)
        };
        // ======================================================================================== //
        //   Update entity
        // ======================================================================================== //
        let mut reqs_entity = LinkedList::new();
        let count = self.count + 1;
        let score = self.score;
        let (mut enemy, mut reqs_enemy) = self.enemy.update(count as f32);
        reqs_entity.append(&mut reqs_enemy);
        let (mut player, mut reqs_player) = self.player.update(PLAYER_RECT, inp, slow);
        reqs_entity.append(&mut reqs_player);
        let mut bullets = LinkedList::new();
        match state {
            State::Shoot if self.stage == 1 => {
                bullets.append(&mut create_stage1_bullet(count, &player, &enemy))
            }
            _ => (),
        }
        for i in self.bullets {
            let bullet_n = i.update(BULLET_RECT);
            if let Some((n, mut reqs)) = bullet_n {
                let (player_n, is_hit, _) = player.check_hit(n.pos, n.r);
                player = player_n;
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
        reqs_ui.push_back(
            TextDesc::new()
                .set_text(format!("{:>012}", self.score))
                .set_rect([300.0, 1280.0, 0.0, 960.0])
                .set_format(TextFormat::Score)
                .pack(),
        );
        let log = match state {
            State::Start(n) if self.stage == 1 => Some(get_stage1_start_log(n)),
            _ => None,
        };
        if let Some(n) = log {
            reqs_ui.push_back(
                TextDesc::new()
                    .set_text(n)
                    .set_rect(LOG_RECT)
                    .set_format(TextFormat::Normal)
                    .pack(),
            );
        }
        // Finish
        let mut reqs = reqs_entity;
        reqs.append(&mut reqs_ui);
        (
            Scene::Stage(Stage {
                stage: self.stage,
                count,
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
