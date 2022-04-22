use super::*;

const P_SPD: f32 = 8.0;

pub struct Stage {
    score: u64,
    count: u32,
    e_hp: [u32; 2],
    e_pos_xy: [f32; 2],
    p_pos_xy: [f32; 2],
}
impl Stage {
    pub fn new() -> Self {
        Self {
            score: 0,
            count: 0,
            e_hp: [2000; 2],
            e_pos_xy: [0.0, 280.0],
            p_pos_xy: [0.0, -280.0],
        }
    }
    pub fn update(self, keystates: &KeyStates) -> (Scene, LinkedList<Request>) {
        // Update player
        let inp_x = (keystates.right > 0) as i32 - (keystates.left > 0) as i32;
        let inp_y = (keystates.up > 0) as i32 - (keystates.down > 0) as i32;
        let c_spd = if keystates.s > 0 { 0.5 } else { 1.0 }
            / if inp_x.abs() + inp_y.abs() == 2 {
                std::f32::consts::SQRT_2
            } else {
                1.0
            };
        let p_pos_xy = [
            self.p_pos_xy[0] + inp_x as f32 * P_SPD * c_spd,
            self.p_pos_xy[1] + inp_y as f32 * P_SPD * c_spd,
        ];
        // Update enemy
        let e_pos_xy = [
            self.e_pos_xy[0],
            self.e_pos_xy[1] + (self.count as f32 * 6.0).to_radians().cos() * 0.5,
        ];
        let e_hp = self.e_hp;
        // Update count
        let count = self.count + 1;
        // Update score
        let score = self.score + 10;
        //
        // Build request list
        let mut reqs = LinkedList::new();
        // Enemy
        reqs.push_back(
            CDataDiff::new()
                .set_trs(e_pos_xy)
                .set_scl([100.0, 100.0])
                .pack(),
        );
        reqs.push_back(Request::DrawImage);
        // Player
        reqs.push_back(
            CDataDiff::new()
                .set_trs(p_pos_xy)
                .set_scl([80.0, 80.0])
                .pack(),
        );
        reqs.push_back(Request::DrawImage);
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
                score,
                count,
                e_hp,
                e_pos_xy,
                p_pos_xy,
            }),
            reqs,
        )
    }
}
