/// [essensial]
pub mod component;
/// [essensial]
/// This provides a way to get fps.
mod fps;
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
    pub fn update(self, reqs: &mut Vec<request::Request>, input: &component::Input) -> Self {
        let scene = self.0.update(reqs, input);
        let fpsdata = self.1.update();
        reqs.push(request::Request::DrawText(
            request::text::TextDesc::new()
                .set_text(format!("{:.1}fps", fpsdata.get_fps()))
                .set_rect([0.0, 1270.0, 920.0, 960.0])
                .set_format(request::text::TextFormat::Fps)
                .set_align(request::text::TextAlign::Right),
        ));
        Self(scene, fpsdata)
    }
}
