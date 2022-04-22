use super::*;

const P_SPD: f32 = 8.0;
const INIT_POS: [f32; 2] = [0.0, -280.0];
const SQUARE_SIZE: f32 = 80.0;

pub struct Player {
    pub pos: [f32; 2],
    pub inp: [i32; 2],
    pub slow: bool,
    pub graze: u32,
}
impl Player {
    pub fn new() -> Self {
        Self {
            pos: INIT_POS,
            inp: [0; 2],
            slow: false,
            graze: 0,
        }
    }
    pub fn update(self, rect: [f32; 4], inp: [i32; 2], slow: bool) -> (Self, LinkedList<Request>) {
        let c_spd = if slow { 0.5 } else { 1.0 }
            / if inp[0].abs() + inp[1].abs() == 2 {
                std::f32::consts::SQRT_2
            } else {
                1.0
            };
        let pos = [
            self.pos[0] + inp[0] as f32 * P_SPD * c_spd,
            self.pos[1] + inp[1] as f32 * P_SPD * c_spd,
        ];
        let pos = [
            pos[0].max(rect[0]).min(rect[1]),
            pos[1].max(rect[3]).min(rect[2]),
        ];
        let mut reqs = LinkedList::new();
        reqs.push_back(
            CDataDiff::new()
                .set_trs(self.pos)
                .set_scl([SQUARE_SIZE, SQUARE_SIZE])
                .pack(),
        );
        reqs.push_back(Request::DrawImage);
        (
            Self {
                pos,
                inp,
                slow,
                graze: self.graze,
            },
            reqs,
        )
    }
    pub fn check_hit(&self, pos: [f32; 2], r: [f32; 2]) -> (bool, bool) {
        let dis = (self.pos[0] - pos[0]).powf(2.0) + (self.pos[1] - pos[1]).powf(2.0);
        if dis < r[0] {
            (true, false)
        } else if dis < r[1] {
            (false, true)
        } else {
            (false, false)
        }
    }
}
