use super::*;

use super::stage::Stage;

const REQUESTS_SIZE: usize = 4;

pub(in super::super) struct Title(u32);
impl Title {
    pub(super) fn new() -> Self {
        Self(0)
    }
    pub(super) fn update(self, keystates: &KeyStates) -> (Scene, Vec<Request>) {
        let mut reqs = Vec::with_capacity(REQUESTS_SIZE);
        reqs.push(
            TextDesc::new()
                .set_text("PRESS ANY KEY TO START")
                .set_rect([0.0, 1280.0, 720.0, 960.0])
                .set_rgba([1.0, 1.0, 1.0, (self.0 as f32).to_radians().cos().abs()])
                .set_format(TextFormat::Score)
                .set_align(TextAlign::Center)
                .pack(),
        );
        if keystates.z == 1 {
            return (Scene::Stage(Stage::new()), reqs);
        }
        (Scene::Title(Self(self.0 + 1)), reqs)
    }
}
