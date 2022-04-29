use super::*;

#[derive(Default)]
pub struct Velocity {
    pub direction: Vector,
    pub speed: f32,
}
impl SystemImpl<CContainer<Position>, CContainer<Velocity>> for System {
    fn process(update: &mut CContainer<Position>, refer: &CContainer<Velocity>) {
        for (k, v) in refer {
            if let Some(mut n) = update.get_mut(k) {
                n.x += v.direction.x * v.speed;
                n.y += v.direction.y * v.speed;
                n.z += v.direction.z * v.speed;
            }
        }
    }
}
