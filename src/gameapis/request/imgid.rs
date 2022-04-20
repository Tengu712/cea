use super::*;

#[derive(Clone, Copy)]
pub enum ImgID {
    Title,
}
impl PackingRequest for ImgID {
    fn pack(self) -> Request {
        Request::SetImage(self)
    }
}
