use super::*;

const P_SPD: f32 = 8.0;

pub struct Stage {
    p_pos_xy: [f32; 2],
}
impl Stage {
    pub fn new() -> Self {
        Self { p_pos_xy: [0.0; 2] }
    }
    pub fn update(self, keystates: &KeyStates) -> (Scene, LinkedList<Request>) {
        let inp_x = (keystates.right > 0) as i32 - (keystates.left > 0) as i32;
        let inp_y = (keystates.up > 0) as i32 - (keystates.down > 0) as i32;
        let c_spd = if keystates.s > 0 { 0.5 } else { 1.0 };
        let c_spd = c_spd
            / if inp_x.abs() + inp_y.abs() == 2 {
                std::f32::consts::SQRT_2
            } else {
                1.0
            };
        let p_pos_xy = [
            self.p_pos_xy[0] + inp_x as f32 * P_SPD * c_spd,
            self.p_pos_xy[1] + inp_y as f32 * P_SPD * c_spd,
        ];
        let mut reqs = LinkedList::new();
        reqs.push_back(
            CDataDiff::new()
                .set_trs(p_pos_xy)
                .set_scl([80.0, 80.0])
                .pack(),
        );
        reqs.push_back(Request::DrawImage);
        (Scene::Stage(Self { p_pos_xy }), reqs)
    }
}
