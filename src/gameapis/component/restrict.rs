use super::*;

pub type RestrictRect = Rect3D;
impl SystemImpl<CContainer<Position>, CContainer<RestrictRect>> for System {
    fn process(update: &mut CContainer<Position>, refer: &CContainer<RestrictRect>) {
        for (k, v) in refer {
            if let Some(mut n) = update.get_mut(k) {
                n.x = n.x.max(v.l).min(v.r);
                n.y = n.y.max(v.b).min(v.t);
                n.z = n.z.max(v.n).min(v.f);
            }
        }
    }
}
