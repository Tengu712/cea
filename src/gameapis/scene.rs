mod title;
/// [essensial]
/// use gameapis::player.
mod stage;

use title::Title;
use stage::Stage;

pub enum Scene {
    Title(Title),
    Stage(Stage),
}
