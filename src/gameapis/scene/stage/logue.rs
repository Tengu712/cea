use super::*;

use super::constant::*;

// Stand character
const ST_DISACT_COL: [f32; 4] = [0.8, 0.8, 0.8, 1.0];
const ST_LEFT_ACT_POS: [f32; 2] = [-360.0, -60.0];
const ST_LEFT_ACT_SCL: [f32; 2] = [840.0, 840.0];
const ST_LEFT_DISACT_POS: [f32; 2] = [-380.0, -80.0];
const ST_LEFT_DISACT_SCL: [f32; 2] = [800.0, 800.0];
const ST_RIGHT_ACT_POS: [f32; 2] = [360.0, -60.0];
const ST_RIGHT_ACT_SCL: [f32; 2] = [840.0, 840.0];
const ST_RIGHT_DISACT_POS: [f32; 2] = [380.0, -80.0];
const ST_RIGHT_DISACT_SCL: [f32; 2] = [800.0, 800.0];
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
    pub(super) fn push_reqs(&self, reqs: &mut Vec<Request>, stage: usize) {
        if self.is_end_log(stage) {
            return;
        }
        let (text, imgid_left, imgid_right, is_right) = get_log(stage, self.0);
        if let Some(n) = imgid_left {
            reqs.push(n.pack());
            reqs.push(
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
            reqs.push(Request::DrawImage);
        }
        if let Some(n) = imgid_right {
            reqs.push(n.pack());
            reqs.push(
                if is_right {
                    CDataDiff::new()
                        .set_trs(ST_RIGHT_ACT_POS)
                        .set_scl(ST_RIGHT_ACT_SCL)
                } else {
                    CDataDiff::new()
                        .set_trs(ST_RIGHT_DISACT_POS)
                        .set_scl(ST_RIGHT_DISACT_SCL)
                        .set_col(ST_DISACT_COL)
                }
                .pack(),
            );
            reqs.push(Request::DrawImage);
        }
        reqs.push(Request::UnsetImage);
        reqs.push(
            CDataDiff::new()
                .set_trs(LOGBOX_POS)
                .set_scl(LOGBOX_SCL)
                .set_col(LOGBOX_COL)
                .pack(),
        );
        reqs.push(Request::DrawImage);
        reqs.push(
            TextDesc::new()
                .set_text(text)
                .set_rect(LOG_RECT)
                .set_format(TextFormat::Normal)
                .pack(),
        );
    }
    pub(super) fn is_end_start_log(&self, stage: usize) -> bool {
        is_end_start_log(stage, self.0)
    }
    pub(super) fn is_end_log(&self, stage: usize) -> bool {
        is_end_log(stage, self.0)
    }
}
