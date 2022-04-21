/// [essential]
/// This defines CDataDiff struct.
pub mod cdata;
/// [essential]
/// This defines ImgID enum that's a kind of resource identifier.
pub mod imgid;
/// [essential]
/// This defines Text struct.
pub mod text;

pub enum Request {
    NoRequest,
    SetImage(imgid::ImgID),
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

pub struct Requests(Vec<Request>);
impl Requests {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn push<T: PackingRequest>(&mut self, arg: T) {
        self.push(arg.pack());
    }
    pub fn get_array(self) -> Vec<Request> {
        self.0
    }
}
