use super::*;

const FLOOR_SQUARE_SIZE: f32 = 350.0;

pub(super) struct Background(i32);
impl Background {
    pub(super) fn new() -> Self {
        Self(0)
    }
    pub(super) fn update(self, lr: i32, is_slow: bool) -> Self {
        let abs = self.0.abs();
        if is_slow {
            Self(self.0)
        } else if lr != 0 {
            Self((self.0 + lr * 5).min(60).max(-60))
        } else if abs == 0 {
            Self(0)
        } else {
            Self(self.0 / abs * (abs - 5))
        }
    }
    pub(super) fn create_reqs(&self, cnt: u32) -> LinkedList<Request> {
        let rot_y = -(self.0 as f32 / 10.0).to_radians();
        let rot_z = (self.0 as f32 / 20.0).to_radians();
        let mut reqs = LinkedList::new();
        reqs.push_back(
            PerseDesc {
                w: WIDTH,
                h: HEIGHT,
                theta: 45.0f32.to_radians(),
                n: 1.0,
                f: 1000.0,
            }
            .pack(),
        );
        reqs.push_back(
            ViewDesc {
                pos: [0.0, (cnt % (FLOOR_SQUARE_SIZE as u32)) as f32, 0.0],
                rot: [-30.0f32.to_radians(), rot_y, rot_z],
            }
            .pack(),
        );
        reqs.push_back(IMGID_FLOOR.pack());
        reqs.push_back(
            CDataDiff::new()
                .set_trs_xyz([0.0, 0.0, 50.0])
                .set_scl([FLOOR_SQUARE_SIZE, FLOOR_SQUARE_SIZE])
                .pack(),
        );
        reqs.push_back(Request::DrawImage);
        reqs.push_back(
            CDataDiff::new()
                .set_trs_xyz([0.0, FLOOR_SQUARE_SIZE, 50.0])
                .set_scl([FLOOR_SQUARE_SIZE, FLOOR_SQUARE_SIZE])
                .pack(),
        );
        reqs.push_back(Request::DrawImage);
        reqs.push_back(
            CDataDiff::new()
                .set_trs_xyz([0.0, FLOOR_SQUARE_SIZE * 2.0, 50.0])
                .set_scl([FLOOR_SQUARE_SIZE, FLOOR_SQUARE_SIZE])
                .pack(),
        );
        reqs.push_back(Request::DrawImage);
        reqs.push_back(
            OrthoDesc {
                l: 0.0,
                r: WIDTH,
                t: HEIGHT,
                b: 0.0,
                n: 0.0,
                f: 1.0,
            }
            .pack(),
        );
        reqs.push_back(
            ViewDesc {
                pos: [0.0; 3],
                rot: [0.0; 3],
            }
            .pack(),
        );
        reqs
    }
}
