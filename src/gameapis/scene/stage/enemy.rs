use super::*;

const ENEMY_SQUARE_SIZE: f32 = 140.0;

pub(super) struct Enemy {
    pub(super) cnt: u32,
    pub(super) pos: [f32; 2],
}
impl Enemy {
    pub(super) fn new() -> Self {
        Self {
            cnt: 0,
            pos: [0.0, 280.0],
        }
    }
    pub(super) fn update(self) -> Self {
        let pos = [self.pos[0], self.pos[1]];
        Self {
            cnt: self.cnt + 1,
            pos,
        }
    }
    pub(super) fn create_body_reqs(&self) -> LinkedList<Request> {
        let mut reqs = LinkedList::new();
        let trs = [
            self.pos[0],
            self.pos[1] + (self.cnt as f32 * 4.0).to_radians().cos() * 10.0,
        ];
        reqs.push_back(IMGID_REMILIA_F0.pack());
        reqs.push_back(
            CDataDiff::new()
                .set_trs(trs)
                .set_scl([ENEMY_SQUARE_SIZE, ENEMY_SQUARE_SIZE])
                .pack(),
        );
        reqs.push_back(Request::DrawImage);
        reqs
    }
}
