/// [essensial]
/// This defines request.
pub mod request;
/// [essensial]
/// This defines each scene updater.
pub mod scene;

use request::{text::*, *};
use scene::*;
use std::collections::LinkedList;

pub struct Game {
    scene: Scene,
    option: Vec<String>,
}
impl Game {
    pub fn new() -> Self {
        Self {
            scene: Scene::Title,
            option: Vec::new(),
        }
    }
    pub fn update(self) -> (Self, LinkedList<Request>) {
        // Update scene
        let next = match self.scene {
            Scene::Title => self.update_title(),
        };
        let mut reqs = LinkedList::new();
        // Background
        // Option
        for (i, v) in next.option.iter().enumerate() {
            reqs.push_back(
                TextDesc::new()
                    .set_text(v)
                    .set_rect([
                        0.0,
                        1280.0,
                        340.0 - 40.0 * (next.option.len() - 1) as f32 / 2.0 + 40.0 * i as f32,
                        720.0,
                    ])
                    .set_format(TextFormat::Option)
                    .set_align(TextAlign::Center)
                    .pack(),
            );
        }
        (next, reqs)
    }
}
