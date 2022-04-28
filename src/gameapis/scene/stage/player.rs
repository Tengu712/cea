use super::*;

use super::bullet::*;

const RECT: Rect = Rect {
    l: GAME_LEFT + 10.0,
    r: GAME_RIGHT - 10.0,
    t: GAME_TOP - 150.0,
    b: GAME_BOTTOM + 20.0,
    n: 0.0,
    f: 0.0,
};
const P_SPD: f32 = 8.0;
const INIT_POS: [f32; 2] = [0.0, -280.0];
const SQUARE_SIZE: f32 = 100.0;
const HITCIRCLE_SIZE: f32 = 16.0;
const SLOWCIRCLE_SIZE: f32 = 140.0;
const PLAYER_BULLET_VELOCITY: f32 = 40.0;
const PLAYER_BULLET_POS_DIF: [f32; 2] = [20.0, 50.0];
const PLAYER_BULLET_BASE_DAMAGE: i32 = 100;

pub(super) struct Player {
    pub(super) pos: Position,
}
impl Player {
    pub(super) fn new() -> Self {
        Self {
            pos: Position {
                x: INIT_POS[0],
                y: INIT_POS[1],
                z: 0.0,
            },
        }
    }
    pub(super) fn update(self, input: &Input) -> Self {
        let vel = Velocity::from_input(input);
        let spd = P_SPD * if input.s > 0 { 0.5 } else { 1.0 };
        let pos = self.pos.update_with_velocity(&vel, spd);
        let pos = pos.restrict(&RECT);
        Self { pos }
    }
    pub(super) fn die(self) -> Self {
        Player::new()
    }
    pub(super) fn push_body_reqs(&self, reqs: &mut Vec<Request>) {
        reqs.push(IMGID_FLAN_B0.pack());
        reqs.push(
            CDataDiff::new()
                .set_trs([self.pos.x, self.pos.y])
                .set_scl([SQUARE_SIZE, SQUARE_SIZE])
                .pack(),
        );
        reqs.push(Request::DrawImage);
    }
    pub(super) fn push_slow_reqs(&self, reqs: &mut Vec<Request>) {
        /*if self.inp.cnt_s <= 0 {
            return;
        }
        reqs.push(IMGID_HITCIRCLE.pack());
        reqs.push(
            CDataDiff::new()
                .set_trs(self.pos)
                .set_scl([HITCIRCLE_SIZE, HITCIRCLE_SIZE])
                .set_rot([0.0, 0.0, (self.inp.cnt_s as f32 * 2.0).to_radians()])
                .pack(),
        );
        reqs.push(Request::DrawImage);
        reqs.push(IMGID_SLOWCIRCLE.pack());
        if self.inp.cnt_s < 10 {
            let size = (SLOWCIRCLE_SIZE + 1.0) * 2.0 * (1.0 - self.inp.cnt_s as f32 / 10.0);
            reqs.push(
                CDataDiff::new()
                    .set_trs(self.pos)
                    .set_scl([size, size])
                    .pack(),
            );
            reqs.push(Request::DrawImage);
        } else {
            reqs.push(
                CDataDiff::new()
                    .set_trs(self.pos)
                    .set_scl([SLOWCIRCLE_SIZE, SLOWCIRCLE_SIZE])
                    .set_rot([0.0, 0.0, (self.inp.cnt_s as f32 * 4.0).to_radians()])
                    .pack(),
            );
            reqs.push(Request::DrawImage);
            reqs.push(
                CDataDiff::new()
                    .set_trs(self.pos)
                    .set_scl([SLOWCIRCLE_SIZE, SLOWCIRCLE_SIZE])
                    .set_rot([0.0, 0.0, -1.0 * (self.inp.cnt_s as f32 * 4.0).to_radians()])
                    .pack(),
            );
            reqs.push(Request::DrawImage);
        }
        */
    }
    pub(super) fn shoot(&self, p_buls: &mut PlayerBullets) {
        if !self.is_shootable() {
            return;
        }
        p_buls.push(
            Bullet::new(BUL_FLAN)
                .set_deg(90.0)
                .set_vel(PLAYER_BULLET_VELOCITY)
                .set_dmg(PLAYER_BULLET_BASE_DAMAGE)
                .set_pos([
                    self.pos.x - PLAYER_BULLET_POS_DIF[0],
                    self.pos.y + PLAYER_BULLET_POS_DIF[1],
                ]),
        );
        p_buls.push(
            Bullet::new(BUL_FLAN)
                .set_deg(90.0)
                .set_vel(PLAYER_BULLET_VELOCITY)
                .set_dmg(PLAYER_BULLET_BASE_DAMAGE)
                .set_pos([
                    self.pos.x + PLAYER_BULLET_POS_DIF[0],
                    self.pos.y + PLAYER_BULLET_POS_DIF[1],
                ]),
        );
    }
    pub(super) fn is_shootable(&self) -> bool {
        //self.inp.cnt_z % 6 == 1
        false
    }
}
