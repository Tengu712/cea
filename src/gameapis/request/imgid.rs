use super::*;

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum ImgID {
    Title,
    FlanB0,
    FlanL0,
    FlanR0,
    RemiliaF0,
    StFlan,
    UiFrame,
}
impl PackingRequest for ImgID {
    fn pack(self) -> Request {
        Request::SetImage(self)
    }
}
