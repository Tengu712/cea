use super::*;

pub struct SamePosition {
    pub targetkey: EntityKey,
    pub targetposition: Position,
}
impl SystemImpl<CContainer<SamePosition>, (&CContainer<Position>, &Entities)> for System {
    fn process(update: &mut CContainer<SamePosition>, refer: &(&CContainer<Position>, &Entities)) {
        let (positions, entities) = refer;
        for v in update.values_mut() {
            if let Some(id) = entities.get(v.targetkey) {
                if let Some(n) = positions.get(id) {
                    v.targetposition.x = n.x;
                    v.targetposition.y = n.y;
                    v.targetposition.z = n.z;
                }
            }
        }
    }
}
impl SystemImpl<CContainer<Position>, CContainer<SamePosition>> for System {
    fn process(update: &mut CContainer<Position>, refer: &CContainer<SamePosition>) {
        for (k, v) in refer {
            if let Some(mut n) = update.get_mut(k) {
                n.x = v.targetposition.x;
                n.y = v.targetposition.y;
                n.z = v.targetposition.z;
            }
        }
    }
}
