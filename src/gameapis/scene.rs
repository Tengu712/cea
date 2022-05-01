use super::asset::*;
use super::entity::*;
use super::system::*;
use super::*;

pub trait Scene {
    fn update(&mut self, world: &mut World) -> Option<Box<dyn Scene>>;
}

#[derive(Default)]
pub struct Title {}
impl Title {
    pub fn new(world: &mut World) -> Box<dyn Scene> {
        world.clear();
        create_fps(&mut world.manager);
        create_title_text(&mut world.manager);
        world.systems.push(system_update_counter);
        world.systems.push(system_fpsmeasure);
        world.systems.push(script_title_text);
        Box::new(Title {})
    }
}
impl Scene for Title {
    fn update(&mut self, world: &mut World) -> Option<Box<dyn Scene>> {
        Some(Stage::new(world))
    }
}

#[derive(Default)]
pub struct Stage {}
impl Stage {
    pub fn new(world: &mut World) -> Box<dyn Scene> {
        world.clear();
        create_fps(&mut world.manager);
        world.systems.push(system_fpsmeasure);
        Box::new(Stage {})
    }
}
impl Scene for Stage {
    fn update(&mut self, _: &mut World) -> Option<Box<dyn Scene>> {
        None
    }
}
