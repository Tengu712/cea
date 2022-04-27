/// [essential]
/// This defines CDataDiff struct.
pub mod cdata;
/// [essential]
/// This defines ImgID enum that's a kind of resource identifier.
pub mod imgid;
/// [essential]
/// This defines Text struct.
pub mod text;

pub const REQUESTS_SIZE: usize = 2048;

pub enum Request {
    SetImage(imgid::ImgID),
    UnsetImage,
    SetCData(cdata::CDataDiff),
    SetView(cdata::ViewDesc),
    SetPerse(cdata::PerseDesc),
    SetOrtho(cdata::OrthoDesc),
    Overlay,
    Multiple,
    DrawImage,
    DrawText(text::TextDesc),
}
pub trait PackingRequest {
    fn pack(self) -> Request;
}
impl PackingRequest for Request {
    fn pack(self) -> Request {
        self
    }
}
