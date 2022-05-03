use super::*;

#[derive(Default)]
pub struct ValueSprite {
    pub format: Option<fn(&Counter) -> Sprite>,
}
