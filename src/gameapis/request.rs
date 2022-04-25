/// [essential]
/// This defines CDataDiff struct.
mod cdata;
/// [essential]
/// This defines ImgID enum that's a kind of resource identifier.
mod imgid;
/// [essential]
/// This defines Text struct.
mod text;

pub enum Request {
    NoRequest,
    SetImage(imgid::ImgID),
    UnsetImage,
    SetCData(cdata::CDataDiff),
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
