use super::*;

pub struct SpriteAnimator {
    pub is_active: bool,
    pub count: u32,
    pub f: fn(&mut Self, &mut Sprite),
}
pub struct TextAnimator {
    pub is_active: bool,
    pub count: u32,
    pub f: fn(&mut Self, &mut Text),
}
