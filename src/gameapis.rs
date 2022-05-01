pub mod asset;
pub mod component;
pub mod entity;
pub mod scene;
pub mod system;

use std::any::type_name;
use std::collections::HashMap;
use std::collections::HashSet;

pub type EntityID = usize;
pub type ScriptKey = &'static str;

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

#[derive(Default)]
pub struct Camera {
    pub pos: component::Vector,
    pub rot: component::Vector,
}

#[derive(Default)]
pub struct World {
    pub manager: entity::EntityManager,
    pub systems: Vec<fn(&mut entity::EntityManager)>,
}
impl World {
    pub fn update(&mut self, input: &Input) {
        self.manager.input = input.clone();
        for system in &self.systems {
            system(&mut self.manager);
        }
    }
    pub fn clear(&mut self) {
        self.manager = Default::default();
        self.systems = Default::default();
    }
}
