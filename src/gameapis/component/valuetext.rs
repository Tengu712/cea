use super::*;

#[derive(Default)]
pub struct ValueText(pub Option<fn(&Counter, &Text) -> Text>);
