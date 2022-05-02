use super::*;

#[derive(Default)]
pub struct ValueText {
    pub format: Option<fn(&Counter) -> String>,
}
