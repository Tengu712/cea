pub(super) mod stage;
pub(super) mod title;

use super::{
    component::*,
    request::{cdata::*, imgid::*, text::*, *},
};

pub(super) enum Scene {
    Title(title::Title),
    Stage(stage::Stage),
}
impl Scene {
    pub(super) fn update(self, reqs: &mut Vec<Request>, keystates: &Input) -> Self {
        match self {
            Scene::Title(n) => n.update(reqs, keystates),
            Scene::Stage(n) => n.update(reqs, keystates),
        }
    }
}

pub(super) fn create_first_scene() -> Scene {
    Scene::Title(title::Title::new())
}
