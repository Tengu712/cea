use super::*;

#[derive(Default)]
pub struct PlayerAnimation(u32);
impl SystemImpl<CContainer<Sprite>, (&CContainer<PlayerAnimation>, &CContainer<Velocity>)>
    for System
{
    fn process(
        update: &mut CContainer<Sprite>,
        refer: &(&CContainer<PlayerAnimation>, &CContainer<Velocity>),
    ) {
        let (playeranimations, velocities) = refer;
        for (pa_k, _) in playeranimations.iter() {
            if let Some(velocity) = velocities.get(pa_k) {
                if let Some(mut n) = update.get_mut(pa_k) {
                    if velocity.direction.x > 0.0 {
                        n.imgid = Some(IMGID_FLAN_R0);
                    } else if velocity.direction.x < 0.0 {
                        n.imgid = Some(IMGID_FLAN_L0);
                    } else {
                        n.imgid = Some(IMGID_FLAN_B0);
                    }
                }
            }
        }
    }
}
