pub mod asset;
pub mod component;
pub mod entity;
pub mod system;

use asset::*;
use component::*;
use entity::*;

use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct Input {
    pub z: i16,
    pub x: i16,
    pub s: i16,
    pub left: i16,
    pub up: i16,
    pub right: i16,
    pub down: i16,
}

pub type EntityID = usize;
pub type EntityKey = &'static str;
pub type CContainer<T> = HashMap<EntityID, T>;
pub type Entities = HashMap<EntityKey, EntityID>;

#[derive(Default)]
pub struct World {
    pub manager: EntityManager,
    pub systems: Vec<fn(&mut EntityManager)>,
}
impl World {
    pub fn update(&mut self, input: &Input) {
        self.manager.input = input.clone();
        for system in &self.systems {
            system(&mut self.manager);
        }
    }
}
