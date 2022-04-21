/// [essensial]
/// This defines request.
pub mod request;
/// [essensial]
/// This defines each scene updater.
pub mod scene;

use request::*;
use std::collections::LinkedList;

pub struct Game {}
impl Game {
    pub fn new() -> Self {
        Self {}
    }
    pub fn update(self) -> (Self, LinkedList<Request>) {
        let mut reqs = LinkedList::new();
        reqs.push_back(text::TextDesc::new().set_text("秘封俱楽部").pack());
        reqs.push_back(cdata::CDataDiff::new().set_scl([640.0, 360.0]).pack());
        reqs.push_back(Request::DrawImage.pack());
        (Self {}, reqs)
    }
}
