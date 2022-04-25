/// [essensial]
/// This provides a way to get fps.
mod fps;
/// [essensial]
/// This provides a way to get key input states.
pub mod input;
/// [essensial]
/// This defines request.
pub mod request;
/// [essensial]
/// This defines each scene updater.
mod scene;

pub struct Game(scene::Scene, fps::FpsData);
impl Game {
    pub fn new() -> Self {
        Self(scene::create_first_scene(), fps::FpsData::new())
    }
    pub fn update(
        self,
        keystates: &input::KeyStates,
    ) -> (Self, std::collections::LinkedList<request::Request>) {
        let (scene, mut reqs) = self.0.update(keystates);
        let fpsdata = self.1.update();
        reqs.push_back(request::Request::DrawText(
            request::text::TextDesc::new()
                .set_text(format!("{:.1}fps", fpsdata.get_fps()))
                .set_rect([0.0, 1270.0, 920.0, 960.0])
                .set_format(request::text::TextFormat::Fps)
                .set_align(request::text::TextAlign::Right),
        ));
        (Self(scene, fpsdata), reqs)
    }
}
