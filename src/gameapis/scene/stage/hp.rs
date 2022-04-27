use super::*;

use super::constant::*;

const HP_GAGE_R: f32 = 150.0;
const HP_GAGE_SQUARE_SIZE: f32 = 4.0;

pub(super) struct Hp(i32, i32);
impl Hp {
    pub(super) fn new(stage: usize, phase: usize) -> Self {
        let max_hp = get_max_hp(stage, phase);
        Self(max_hp, max_hp)
    }
    pub(super) fn update(self, damage_sum: i32) -> Self {
        Self(self.0 - damage_sum, self.1)
    }
    pub(super) fn is_dead(&self) -> bool {
        self.0 <= 0
    }
    pub(super) fn push_reqs(&self, reqs: &mut Requests, e_pos: [f32; 2]) {
        let theta = 360.0 * self.0.max(0) as f32 / self.1.max(1) as f32;
        reqs.push(Request::UnsetImage);
        for i in 0..360 {
            if i as f32 >= theta {
                break;
            }
            reqs.push(
                CDataDiff::new()
                    .set_trs([
                        e_pos[0] - HP_GAGE_R * (i as f32).to_radians().sin(),
                        e_pos[1] + HP_GAGE_R * (i as f32).to_radians().cos(),
                    ])
                    .set_rot([0.0, 0.0, (i as f32).to_radians()])
                    .set_scl([HP_GAGE_SQUARE_SIZE, HP_GAGE_SQUARE_SIZE])
                    .pack(),
            );
            reqs.push(Request::DrawImage);
        }
    }
}
