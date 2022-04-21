/// [essensial]
/// This provides a way to get key input states.
pub mod input;
/// [essensial]
/// This defines request.
pub mod request;
/// [essensial]
/// This defines each scene updater.
pub mod scene;

pub struct Game(scene::Scene);
impl Game {
    pub fn new() -> Self {
        Self(scene::Scene::Title(scene::title::Title::new()))
    }
    pub fn update(
        self,
        keystates: &input::KeyStates,
    ) -> (Self, std::collections::LinkedList<request::Request>) {
        let (scene, reqs) = match self.0 {
            scene::Scene::Title(n) => n.update(keystates),
            scene::Scene::Stage(n) => n.update(keystates),
        };
        (Self(scene), reqs)
    }
}
