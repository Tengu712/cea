use super::*;

use super::stage1::*;

// Stand character
const ST_DISACT_COL: [f32; 4] = [0.8, 0.8, 0.8, 1.0];
const ST_LEFT_ACT_POS: [f32; 2] = [-360.0, -60.0];
const ST_LEFT_ACT_SCL: [f32; 2] = [840.0, 840.0];
const ST_LEFT_DISACT_POS: [f32; 2] = [-380.0, -80.0];
const ST_LEFT_DISACT_SCL: [f32; 2] = [800.0, 800.0];
// Logue
const LOGBOX_POS: [f32; 2] = [0.0, -300.0];
const LOGBOX_SCL: [f32; 2] = [(GAME_RIGHT - GAME_LEFT) as f32 * 0.85, 120.0];
const LOGBOX_COL: [f32; 4] = [0.0, 0.0, 0.0, 0.8];
const LOG_RECT: [f32; 4] = [340.0, 940.0, 735.0, HEIGHT];

pub(super) struct Logue(usize);
impl Logue {
    pub(super) fn new() -> Self {
        Self(0)
    }
    pub(super) fn update(self, cnt_z: i16) -> Self {
        if cnt_z == 1 {
            Self(self.0 + 1)
        } else {
            Self(self.0)
        }
    }
    pub(super) fn create_reqs(&self, stage: usize) -> LinkedList<Request> {
        let mut reqs = LinkedList::new();
        if self.is_end_log(stage) {
            return reqs;
        }
        let (text, imgid_left, is_right) = LOG[stage][self.0].clone();
        if let Some(n) = imgid_left {
            reqs.push_back(n.pack());
            reqs.push_back(
                if is_right {
                    CDataDiff::new()
                        .set_trs(ST_LEFT_DISACT_POS)
                        .set_scl(ST_LEFT_DISACT_SCL)
                        .set_col(ST_DISACT_COL)
                } else {
                    CDataDiff::new()
                        .set_trs(ST_LEFT_ACT_POS)
                        .set_scl(ST_LEFT_ACT_SCL)
                }
                .pack(),
            );
            reqs.push_back(Request::DrawImage);
        }
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
                .set_text(text)
                .set_rect(LOG_RECT)
                .set_format(TextFormat::Normal)
                .pack(),
        );
        reqs
    }
    pub(super) fn is_end_start_log(&self, stage: usize) -> bool {
        stage >= STAGE_SIZE || self.0 >= LOG_MAX_SIZE || self.0 >= START_LOG_SIZE[stage]
    }
    pub(super) fn is_end_log(&self, stage: usize) -> bool {
        stage >= STAGE_SIZE || self.0 >= LOG_MAX_SIZE || self.0 >= END_LOG_SIZE[stage]
    }
}
