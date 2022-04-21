pub mod title;
pub mod stage;

use super::{
    input::KeyStates,
    request::{cdata::*, text::*, *},
};
use std::collections::LinkedList;
use title::Title;
use stage::Stage;

pub enum Scene {
    Title(Title),
    Stage(Stage),
}
