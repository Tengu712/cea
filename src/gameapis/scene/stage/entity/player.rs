use super::*;

const PLAYER_RECT: [f32; 4] = [
    GAME_LEFT + 50.0,
    GAME_RIGHT - 50.0,
    GAME_TOP - 200.0,
    GAME_BOTTOM + 80.0,
];
const P_SPD: f32 = 8.0;
const INIT_POS: [f32; 2] = [0.0, -280.0];
const SQUARE_SIZE: f32 = 80.0;

pub struct PlayerInput {
    pub lr_ud: [i32; 2],
    pub slow: bool,
    pub snap: bool,
    pub atack: bool,
}

pub struct Player {
    pub pos: [f32; 2],
    pub inp: PlayerInput,
}
impl Player {
    pub fn new() -> Self {
        Self {
            pos: INIT_POS,
            inp: PlayerInput {
                lr_ud: [0; 2],
                slow: false,
                snap: false,
                atack: false,
            },
        }
    }
    pub fn update(self, inp: PlayerInput) -> Self {
        let c_spd = if inp.slow { 0.5 } else { 1.0 }
            / if inp.lr_ud[0].abs() + inp.lr_ud[1].abs() == 2 {
                std::f32::consts::SQRT_2
            } else {
                1.0
            };
        let pos = [
            self.pos[0] + inp.lr_ud[0] as f32 * P_SPD * c_spd,
            self.pos[1] + inp.lr_ud[1] as f32 * P_SPD * c_spd,
        ];
        let pos = [
            pos[0].max(PLAYER_RECT[0]).min(PLAYER_RECT[1]),
            pos[1].max(PLAYER_RECT[3]).min(PLAYER_RECT[2]),
        ];
        Self { pos, inp }
    }
    pub fn create_reqs_body(&self) -> LinkedList<Request> {
        let mut reqs = LinkedList::new();
        reqs.push_back(
            CDataDiff::new()
                .set_trs(self.pos)
                .set_scl([SQUARE_SIZE, SQUARE_SIZE])
                .pack(),
        );
        reqs.push_back(Request::DrawImage);
        reqs
    }
    pub fn check_hit(&self, pos: [f32; 2], r: f32) -> bool {
        if (self.pos[0] - pos[0]).powf(2.0) + (self.pos[1] - pos[1]).powf(2.0) < r.powf(2.0) {
            true
        } else {
            false
        }
    }
}
