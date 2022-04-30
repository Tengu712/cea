use super::*;

pub fn title_text_animation(animator: &mut TextAnimator, text: &mut Text) {
    animator.count += 1;
    text.rgba.w = (animator.count as f32).to_radians().cos().abs();
}
