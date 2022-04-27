pub(super) mod stage;
pub(super) mod title;

use super::{
    input::KeyStates,
    request::{cdata::*, imgid::*, text::*, *},
};
use std::collections::LinkedList;

pub(super) enum Scene {
    Title(title::Title),
    Stage(stage::Stage),
}
impl Scene {
    pub(super) fn update(self, keystates: &KeyStates) -> (Self, Vec<Request>) {
        match self {
            Scene::Title(n) => n.update(keystates),
            Scene::Stage(n) => n.update(keystates),
        }
    }
}

pub(super) fn create_first_scene() -> Scene {
    Scene::Title(title::Title::new())
}
