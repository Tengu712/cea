use super::*;

const REQUESTS_SIZE: usize = 128;

pub(super) struct Requests(pub(super) Vec<Request>);
impl Requests {
    pub(super) fn new() -> Self {
        Self(Vec::with_capacity(REQUESTS_SIZE))
    }
    pub(super) fn push(&mut self, req: Request) {
        if self.0.len() >= REQUESTS_SIZE {
            return;
        }
        self.0.push(req);
    }
}
