use super::*;

const ENEMY_SQUARE_SIZE: f32 = 128.0;
const HP_GAGE_R: f32 = 150.0;
const HP_GAGE_SQUARE_SIZE: f32 = 4.0;

pub struct Enemy {
    pub hp: [u32; 2],
    pub pos: [f32; 2],
}
impl Enemy {
    pub fn new() -> Self {
        Self {
            hp: [2000; 2],
            pos: [0.0, 280.0],
        }
    }
    pub fn update(self) -> Self {
        let pos = [self.pos[0], self.pos[1]];
        Self { hp: self.hp, pos }
    }
    pub fn create_reqs_body(&self) -> LinkedList<Request> {
        let mut reqs = LinkedList::new();
        reqs.push_back(ImgID::RemiliaF0.pack());
        reqs.push_back(
            CDataDiff::new()
                .set_trs(self.pos)
                .set_scl([ENEMY_SQUARE_SIZE, ENEMY_SQUARE_SIZE])
                .pack(),
        );
        reqs.push_back(Request::DrawImage);
        reqs
    }
    pub fn create_reqs_hp_gage(&self) -> LinkedList<Request> {
        let mut reqs = LinkedList::new();
        let theta = 360.0 * self.hp[0] as f32 / self.hp[1] as f32;
        reqs.push_back(Request::UnsetImage);
        for i in 0..360 {
            if i as f32 > theta {
                break;
            }
            reqs.push_back(
                CDataDiff::new()
                    .set_trs([
                        self.pos[0] - HP_GAGE_R * (i as f32).to_radians().sin(),
                        self.pos[1] + HP_GAGE_R * (i as f32).to_radians().cos(),
                    ])
                    .set_rot([0.0, 0.0, (i as f32).to_radians()])
                    .set_scl([HP_GAGE_SQUARE_SIZE, HP_GAGE_SQUARE_SIZE])
                    .pack(),
            );
            reqs.push_back(Request::DrawImage);
        }
        reqs
    }
}