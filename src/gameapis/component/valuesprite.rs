use super::*;

#[derive(Default)]
pub struct ValueSprite(pub Option<fn(&Counter) -> Sprite>);
