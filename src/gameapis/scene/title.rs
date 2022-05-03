use super::*;

#[derive(Default)]
pub struct Title;
impl Title {
    pub fn new(world: &mut World) -> Box<dyn Scene> {
        world.clear();
        create_fps(&mut world.emngr);
        create_title_text(&mut world.emngr);
        world.systems.push(system_update_counter);
        world.systems.push(system_fpsmeasure);
        world.systems.push(system_value_text);
        Box::new(Title)
    }
}
impl Scene for Title {
    fn update(&mut self, world: &mut World) -> Option<Box<dyn Scene>> {
        if world.emngr.input.z == 1
            || world.emngr.input.x == 1
            || world.emngr.input.s == 1
            || world.emngr.input.left == 1
            || world.emngr.input.right == 1
            || world.emngr.input.up == 1
            || world.emngr.input.down == 1
        {
            Some(super::stage::Stage::new(world))
        } else {
            None
        }
    }
}
