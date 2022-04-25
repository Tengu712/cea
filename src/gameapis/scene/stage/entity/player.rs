use super::*;

const PLAYER_RECT: [f32; 4] = [
    GAME_LEFT + 10.0,
    GAME_RIGHT - 10.0,
    GAME_TOP - 150.0,
    GAME_BOTTOM + 20.0,
];
const P_SPD: f32 = 8.0;
const INIT_POS: [f32; 2] = [0.0, -280.0];
const SQUARE_SIZE: f32 = 100.0;
const PLAYER_BULLET_VELOCITY: f32 = 40.0;
const PLAYER_BULLET_POS_DIF: [f32; 2] = [20.0, 50.0];
const PLAYER_BULLET_BASE_DAMAGE: i32 = 100;

pub struct PlayerInput {
    pub lr_ud: [i32; 2],
    pub cnt_s: i16,
    pub cnt_z: i16,
    pub cnt_x: i16,
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
                cnt_s: 0,
                cnt_z: 0,
                cnt_x: 0,
            },
        }
    }
    pub fn update(self, inp: PlayerInput) -> Self {
        let c_spd = if inp.cnt_s > 0 { 0.5 } else { 1.0 }
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
        if self.inp.lr_ud[0] == 1 {
            reqs.push_back(ImgID::FlanR0.pack());
        } else if self.inp.lr_ud[0] == -1 {
            reqs.push_back(ImgID::FlanL0.pack());
        } else {
            reqs.push_back(ImgID::FlanB0.pack());
        }
        reqs.push_back(
            CDataDiff::new()
                .set_trs(self.pos)
                .set_scl([SQUARE_SIZE, SQUARE_SIZE])
                .pack(),
        );
        reqs.push_back(Request::DrawImage);
        reqs
    }
    pub fn shoot(&self) -> LinkedList<Bullet> {
        let mut res = LinkedList::new();
        if !self.is_shootable() {
            return res;
        }
        let bul = Bullet::new(BUL_FLAN)
            .set_deg(90.0)
            .set_vel(PLAYER_BULLET_VELOCITY)
            .set_dmg(PLAYER_BULLET_BASE_DAMAGE);
        res.push_back(bul.clone().set_pos([
            self.pos[0] - PLAYER_BULLET_POS_DIF[0],
            self.pos[1] + PLAYER_BULLET_POS_DIF[1],
        ]));
        res.push_back(bul.set_pos([
            self.pos[0] + PLAYER_BULLET_POS_DIF[0],
            self.pos[1] + PLAYER_BULLET_POS_DIF[1],
        ]));
        res
    }
    pub fn is_shootable(&self) -> bool {
        self.inp.cnt_z % 6 == 1
    }
}
