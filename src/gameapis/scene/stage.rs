use super::{
    super::{bullet::Bullet, enemy::Enemy, player::Player},
    *,
};

const GAME_LEFT: f32 = -320.0;
const GAME_RIGHT: f32 = 320.0;
const GAME_TOP: f32 = 480.0;
const GAME_BOTTOM: f32 = -480.0;
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

enum State {
    Start(u32),
    Shoot(u32),
    End(u32),
}

pub struct Stage {
    stage: u32,
    count: u32,
    score: u64,
    pause: bool,
    state: State,
    player: Player,
    enemy: Enemy,
    bullet: LinkedList<Bullet>,
}
impl Stage {
    pub fn new() -> Self {
        Self {
            stage: 0,
            count: 0,
            score: 0,
            pause: false,
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
        //
        //   == Do task that reacts with input ==
        // Pause
        let pause = if keystates.e == 1 {
            !self.pause
        } else {
            self.pause
        };
        let player = if pause {
            self.player
        } else {
            self.player.update(PLAYER_RECT, keystates)
        };
        //
        //   == Calculate information ==
        // Update entities
        let enemy = if pause {
            self.enemy
        } else {
            self.enemy.update(self.count as f32)
        };
        let mut bullet = self
            .bullet
            .into_iter()
            .filter_map(|n| n.update(BULLET_RECT))
            .collect::<LinkedList<Bullet>>();
        if self.count == 1 {
            bullet.push_back(Bullet::new().set_vel(5.0).set_deg(180.0));
        }
        // Update data
        let count = self.count + 1 - pause as u32;
        let score = self.score + 10;
        //
        //   == Build request list ==
        // Enemy
        reqs.push_back(
            CDataDiff::new()
                .set_trs(enemy.pos_xy)
                .set_scl([100.0, 100.0])
                .pack(),
        );
        reqs.push_back(Request::DrawImage);
        // Player
        reqs.push_back(
            CDataDiff::new()
                .set_trs(player.pos)
                .set_scl([80.0, 80.0])
                .pack(),
        );
        reqs.push_back(Request::DrawImage);
        // Bullet
        for i in bullet.iter() {
            reqs.push_back(CDataDiff::new().set_trs(i.pos).set_scl([10.0, 10.0]).pack());
            reqs.push_back(Request::DrawImage);
        }
        // Score
        reqs.push_back(
            TextDesc::new()
                .set_text(format!("{:>012}", score))
                .set_rect([300.0, 1280.0, 0.0, 960.0])
                .set_format(TextFormat::Score)
                .pack(),
        );
        (
            Scene::Stage(Self {
                stage: self.stage,
                count,
                score,
                pause,
                state: self.state,
                player,
                enemy,
                bullet,
            }),
            reqs,
        )
    }
}
