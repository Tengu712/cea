use super::*;

pub type Position = Vector;
impl SystemImpl<CContainer<Sprite>, CContainer<Position>> for System {
    fn process(update: &mut CContainer<Sprite>, refer: &CContainer<Position>) {
        for (k, v) in refer {
            if let Some(mut n) = update.get_mut(k) {
                n.translation = v.clone();
            }
        }
    }
}
