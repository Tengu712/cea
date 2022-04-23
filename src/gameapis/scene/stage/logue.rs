use super::*;

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
                    .set_trs([-380.0, -120.0])
                    .set_scl([720.0, 720.0])
                    .pack(),
            );
            reqs.push_back(Request::DrawImage);
            reqs.push_back(Request::UnsetImage);
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
