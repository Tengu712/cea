use super::*;

pub struct PlayerInput;
impl SystemImpl<CContainer<Velocity>, (&CContainer<PlayerInput>, &Input)> for System {
    fn process(update: &mut CContainer<Velocity>, refer: &(&CContainer<PlayerInput>, &Input)) {
        let (pi_map, input) = refer;
        for (k, _) in pi_map.into_iter() {
            if let Some(mut n) = update.get_mut(k) {
                let lr = (input.right > 0) as i32 - (input.left > 0) as i32;
                let ud = (input.up > 0) as i32 - (input.down > 0) as i32;
                let coef = if lr.abs() + ud.abs() == 2 {
                    1.0 / std::f32::consts::SQRT_2
                } else {
                    1.0
                };
                n.direction.x = lr as f32 * coef;
                n.direction.y = ud as f32 * coef;
            }
        }
    }
}