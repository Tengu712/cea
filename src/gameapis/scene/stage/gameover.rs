use super::*;

const END_COUNT: i32 = 240;
const RECT: [f32; 4] = [0.0, WIDTH, HEIGHT / 2.0 - 50.0, HEIGHT];

pub(super) struct GameOver(i32);
impl GameOver {
    pub(super) fn new() -> Self {
        Self(0)
    }
    pub(super) fn update(self) -> Self {
        Self(self.0 + 1)
    }
    pub(super) fn is_end(&self) -> bool {
        self.0 >= END_COUNT
    }
    pub(super) fn push_reqs(&self, reqs: &mut Vec<Request>) {
        reqs.push(Request::UnsetImage);
        reqs.push(
            CDataDiff::new()
                .set_scl([WIDTH, HEIGHT])
                .set_col([0.0, 0.0, 0.0, (self.0 as f32 / 180.0).min(1.0)])
                .pack(),
        );
        reqs.push(Request::DrawImage);
        reqs.push(
            TextDesc::new()
                .set_text("GAME OVER")
                .set_rect(RECT)
                .set_rgba([1.0, 1.0, 1.0, (self.0 as f32 / 60.0).min(1.0)])
                .set_align(TextAlign::Center)
                .set_format(TextFormat::GameOver)
                .pack(),
        );
    }
}
