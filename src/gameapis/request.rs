/// [essential]
/// This defines CDataDiff struct.
mod cdata;
/// [essential]
/// This defines ImgID enum that's a kind of resource identifier.
mod imgid;
/// [essential]
/// This defines Text struct.
mod text;

use cdata::CDataDiff;
use imgid::ImgID;

#[derive(Clone, Copy)]
pub enum Request {
    NoRequest,
    SetImage(ImgID),
    SetCData(CDataDiff),
    DrawImage,
    DrawText,
}
pub trait PackingRequest {
    fn pack(self) -> Request;
}
impl PackingRequest for Request {
    fn pack(self) -> Request {
        self
    }
}

const REQ_SIZE: usize = 20;
pub struct Requests([Request; REQ_SIZE], usize);
impl Requests {
    pub fn new() -> Self {
        Self([Request::NoRequest; REQ_SIZE], 0)
    }
    pub fn push<T: PackingRequest>(self, arg: T) -> Self {
        if self.1 >= REQ_SIZE {
            return self;
        }
        let mut self_mut = self;
        self_mut.0[self_mut.1] = arg.pack();
        self_mut.1 += 1;
        self_mut
    }
    pub fn get_array(self) -> [Request; REQ_SIZE] {
        self.0
    }
}
