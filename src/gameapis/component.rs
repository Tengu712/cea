use super::*;

pub enum ComponentState {
    Active,
    Disactive,
    Empty,
}
impl Default for ComponentState {
    fn default() -> Self {
        ComponentState::Empty
    }
}
impl ComponentState {
    pub fn is_active(&self) -> bool {
        match self {
            ComponentState::Active => true,
            _ => false,
        }
    }
}

#[derive(Default)]
pub struct CContainer<T> {
    empty_idx: Vec<usize>,
    id_idx: HashMap<EntityID, usize>,
    v: Vec<(EntityID, ComponentState, T)>,
}
impl<T> CContainer<T> {
    /// Insert component to vector empty slot.
    pub fn insert(&mut self, id: EntityID, component: T) {
        match self.empty_idx.pop() {
            Some(idx) => {
                self.id_idx.insert(id, idx);
                self.v[idx] = (id, ComponentState::Active, component);
            }
            None => {
                self.id_idx.insert(id, self.v.len());
                self.v.push((id, ComponentState::Active, component));
            }
        }
    }
    /// Change component state to empty and push empty_idx stack the index.
    pub fn remove(&mut self, id: &EntityID) {
        if let Some(idx) = self.id_idx.remove(id) {
            self.v[idx].1 = ComponentState::Empty;
            self.empty_idx.push(idx);
        }
    }
    pub fn disactive(&mut self, id: &EntityID) {
        if let Some(idx) = self.id_idx.get(id) {
            self.v[*idx].1 = ComponentState::Disactive;
        }
    }
    pub fn active(&mut self, id: &EntityID) {
        if let Some(idx) = self.id_idx.get(id) {
            self.v[*idx].1 = ComponentState::Active;
        }
    }
    /// Get (is_active, component). If the component is disactive, return None.
    pub fn get(&self, id: &EntityID) -> Option<&T> {
        match self.id_idx.get(id) {
            Some(idx) => match self.v.get(*idx) {
                Some((_, s, c)) => match s {
                    ComponentState::Active => Some(c),
                    _ => None,
                },
                None => None,
            },
            None => None,
        }
    }
    /// Get (is_active, component).
    pub fn get_mut(&mut self, id: &EntityID) -> Option<&mut T> {
        match self.id_idx.get(id) {
            Some(idx) => match self.v.get_mut(*idx) {
                Some((_, s, c)) => match s {
                    ComponentState::Active => Some(c),
                    _ => None,
                },
                None => None,
            },
            None => None,
        }
    }
    pub fn iter(&self) -> std::slice::Iter<(EntityID, ComponentState, T)> {
        self.v.iter()
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<(EntityID, ComponentState, T)> {
        self.v.iter_mut()
    }
    pub fn len(&self) -> usize {
        self.v.len()
    }
}

#[derive(Default)]
pub struct Components {
    pub counters: CContainer<Counter>,
    pub fpsmeasures: CContainer<FpsMeasure>,
    pub positions: CContainer<Position>,
    pub removerects: CContainer<RemoveRect>,
    pub restricts: CContainer<RestrictRect>,
    pub sameposition2ds: CContainer<SamePosition2D>,
    pub sprite3ds: CContainer<Sprite>,
    pub sprites: CContainer<Sprite>,
    pub texts: CContainer<Text>,
    pub valuetexts: CContainer<ValueText>,
    pub velocities: CContainer<Velocity>,
}
impl Components {
    pub fn remove(&mut self, id: &EntityID) {
        self.counters.remove(id);
        self.fpsmeasures.remove(id);
        self.positions.remove(id);
        self.removerects.remove(id);
        self.restricts.remove(id);
        self.sameposition2ds.remove(id);
        self.sprite3ds.remove(id);
        self.sprites.remove(id);
        self.texts.remove(id);
        self.valuetexts.remove(id);
        self.velocities.remove(id);
    }
}

#[derive(Default, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[derive(Default, Clone)]
pub struct Vector4D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[derive(Default)]
pub struct Rect {
    pub l: f32,
    pub r: f32,
    pub t: f32,
    pub b: f32,
}
#[derive(Default)]
pub struct Rect3D {
    pub l: f32,
    pub r: f32,
    pub b: f32,
    pub t: f32,
    pub n: f32,
    pub f: f32,
}

// counter
#[derive(Default)]
pub struct Counter {
    pub speed: u64,
    pub count: u64,
    pub count_max: u64,
}
#[derive(Default)]
pub struct ValueText {
    pub format: Option<fn(u64) -> String>,
}
pub struct FpsMeasure {
    pub fps: f32,
    pub count: u32,
    pub last: std::time::Instant,
}
// fpsmeasure
impl Default for FpsMeasure {
    fn default() -> Self {
        Self {
            fps: 0.0,
            count: 0,
            last: std::time::Instant::now(),
        }
    }
}
// movement
pub type Position = Vector;
pub type RestrictRect = Rect3D;
pub type RemoveRect = Rect3D;
#[derive(Default)]
pub struct SamePosition2D(pub EntityID);
#[derive(Default)]
pub struct Velocity {
    pub direction: Vector,
    pub speed: f32,
}
// graphic
#[derive(Default)]
pub struct Sprite {
    pub imgid: Option<&'static str>,
    pub translation: Vector,
    pub rotation: Vector,
    pub scaling: Vector,
    pub color: Vector4D,
}
pub enum TextAlign {
    Left,
    Center,
    Right,
}
impl Default for TextAlign {
    fn default() -> Self {
        TextAlign::Left
    }
}
#[derive(Default)]
pub struct Text {
    pub visible: bool,
    pub text: String,
    pub rect: Rect,
    pub rgba: Vector4D,
    pub fontname: &'static str,
    pub size: f32,
    pub align: TextAlign,
}
