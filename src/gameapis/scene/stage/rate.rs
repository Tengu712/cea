use super::*;

const DELAY: i32 = 60;
const RATE_GAGE_MAX: i32 = 1000;
const RATE_GAGE_R: f32 = 80.0;
const RATE_GAGE_SQUARE_SIZE: f32 = 4.0;

pub(super) struct Rate(pub i32, i32);
impl Rate {
    pub(super) fn new() -> Self {
        Self(0, 0)
    }
    pub(super) fn update(self, cnt_graze: u32, cnt_z: i16) -> Self {
        let rate_delay = if cnt_graze > 0 {
            DELAY
        } else {
            (self.1 - 1).max(0)
        };
        let rate = self.0 + cnt_graze as i32 * 5
            - (cnt_z == 1) as i32 * 50
            - (cnt_z > 0) as i32
            - (rate_delay <= 0) as i32;
        let rate = rate.max(0).min(RATE_GAGE_MAX);
        Self(rate, rate_delay)
    }
    pub(super) fn update_while_hit_fragile(self, is_snap: bool) -> Self {
        if is_snap {
            let rate = self.0 + 100;
            let rate = rate.max(0).min(RATE_GAGE_MAX);
            Self(rate, DELAY)
        } else {
            self
        }
    }
    pub(super) fn die(self) -> Self {
        Self(0, 0)
    }
    pub(super) fn push_reqs(&self, reqs: &mut Vec<Request>, p_pos: [f32; 2]) {
        let theta = 180.0 * self.0.max(0) as f32 / RATE_GAGE_MAX as f32;
        reqs.push(Request::UnsetImage);
        for i in 0..180 {
            if i as f32 >= theta {
                break;
            }
            reqs.push(
                CDataDiff::new()
                    .set_trs([
                        p_pos[0] - RATE_GAGE_R * (i as f32 * 2.0).to_radians().sin(),
                        p_pos[1] + RATE_GAGE_R * (i as f32 * 2.0).to_radians().cos(),
                    ])
                    .set_rot([0.0, 0.0, (i as f32 * 2.0).to_radians()])
                    .set_scl([RATE_GAGE_SQUARE_SIZE, RATE_GAGE_SQUARE_SIZE])
                    .pack(),
            );
            reqs.push(Request::DrawImage);
        }
    }
}
