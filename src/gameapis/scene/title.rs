use super::super::Game;
use super::*;

impl Game {
    pub fn update_title(self) -> Self {
        let mut option = Vec::new();
        option.push(String::from("CONTINUE"));
        Self {
            scene: Scene::Title,
            option,
        }
    }
}
