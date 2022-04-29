/*
use super::*;

#[derive(Default)]
pub struct PlayerSlowAnimation{
    pub knd: PlayerSlowKind,
    pub cnt: u32,
}
impl SystemImpl<CContainer<PlayerSlowAnimation>, Input> for System {
    fn process(update: &mut CContainer<PlayerSlowAnimation>, refer: &Input) {
        for (k, v) in update {
        }
    }
}
impl SystemImpl<CContainer<Sprite>, CContainer<PlayerSlowAnimation>> for System {
    fn process(update: &mut CContainer<Sprite>, refer: &CContainer<PlayerSlowAnimation>) {
        for (k, v) in refer {
            if let Some(mut n) = update.get_mut(k) {
                match v.knd {
                    PlayerSlowKind::Hitbox => n.imgid = Some(IMGID_HITCIRCLE),
                    _ => n.imgid = Some(IMGID_SLOWCIRCLE),
                }
                match v.knd {
                    PlayerSlowKind::Hitbox => n.rotation.z = (v.cnt as f32 * 2.0).to_radians(),
                    _
                    PlayerSlowKind::TurnLeft => 
                }
            }
        }
    }
}
*/