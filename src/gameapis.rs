/// [essensial]
/// This defines request.
pub mod request;
/// [essensial]
/// This defines each scene updater.
pub mod scene;

pub struct Game {}
impl Game {
    pub fn new() -> Self {
        Self {}
    }
    pub fn update(self) -> (Self, request::Requests) {
        (
            Self {},
            request::Requests::new()
                .push(request::text::TextDesc::new().set_text("秘封俱楽部"))
                .push(request::cdata::CDataDiff::new().set_scl([640.0, 360.0]))
                .push(request::Request::DrawImage),
        )
    }
}
