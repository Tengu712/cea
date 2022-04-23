use super::*;

// Stand character
const ST_LEFT_ACT_POS: [f32; 2] = [-360.0, -60.0];
const ST_LEFT_ACT_SCL: [f32; 2] = [840.0, 840.0];
// Logue
const LOGBOX_POS: [f32; 2] = [0.0, -300.0];
const LOGBOX_SCL: [f32; 2] = [(GAME_RIGHT - GAME_LEFT) as f32 * 0.85, 120.0];
const LOGBOX_COL: [f32; 4] = [0.0, 0.0, 0.0, 0.8];
const LOG_RECT: [f32; 4] = [340.0, 940.0, 735.0, HEIGHT];

pub struct Logue(usize);
impl Logue {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn update(self, is_down_z: bool) -> Self {
        if is_down_z {
            Self(self.0 + 1)
        } else {
            Self(self.0)
        }
    }
    pub fn create_reqs(&self, stage: u32) -> LinkedList<Request> {
        let mut reqs = LinkedList::new();
        if stage == 1 {
            let (n, l) = &STAGE1_LOG[self.0];
            reqs.push_back(l.clone().pack());
            reqs.push_back(
                CDataDiff::new()
                    .set_trs(ST_LEFT_ACT_POS)
                    .set_scl(ST_LEFT_ACT_SCL)
                    .pack(),
            );
            reqs.push_back(Request::DrawImage);
            reqs.push_back(Request::UnsetImage);
            reqs.push_back(
                CDataDiff::new()
                    .set_trs(LOGBOX_POS)
                    .set_scl(LOGBOX_SCL)
                    .set_col(LOGBOX_COL)
                    .pack(),
            );
            reqs.push_back(Request::DrawImage);
            reqs.push_back(
                TextDesc::new()
                    .set_text(n)
                    .set_rect(LOG_RECT)
                    .set_format(TextFormat::Normal)
                    .pack(),
            );
        }
        reqs
    }
    pub fn is_end_start_log(&self, stage: u32) -> bool {
        self.0 >= if stage == 1 { STAGE1_START_LOG_SIZE } else { 0 }
    }
    pub fn is_end_log(&self, stage: u32) -> bool {
        self.0 >= if stage == 1 { STAGE1_LOG_SIZE } else { 0 }
    }
}
