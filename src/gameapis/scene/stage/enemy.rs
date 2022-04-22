use super::*;

const ENEMY_SQUARE_SIZE: f32 = 100.0;

pub struct Enemy {
    pub phase: u32,
    pub hp: [u32; 2],
    pub pos: [f32; 2],
}
impl Enemy {
    pub fn new() -> Self {
        Self {
            phase: 0,
            hp: [2000; 2],
            pos: [0.0, 280.0],
        }
    }
    pub fn update(self, cnt_all_f32: f32) -> (Self, LinkedList<Request>) {
        let pos = [
            self.pos[0],
            self.pos[1] + (cnt_all_f32 * 6.0).to_radians().cos() * 0.5,
        ];
        let mut reqs = LinkedList::new();
        reqs.push_back(
            CDataDiff::new()
                .set_trs(self.pos)
                .set_scl([ENEMY_SQUARE_SIZE, ENEMY_SQUARE_SIZE])
                .pack(),
        );
        reqs.push_back(Request::DrawImage);
        (
            Self {
                phase: self.phase,
                hp: self.hp,
                pos,
            },
            reqs,
        )
    }
}