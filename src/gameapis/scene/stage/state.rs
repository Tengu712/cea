use super::*;

pub enum State {
    Start(u32),
    Shoot,
    End(u32),
}
impl State {
    // Update state. If 2nd return value is true, it says go to next stage.
    pub fn update(self, stage: u32, is_z_down: bool) -> (Self, bool) {
        let next = match self {
            State::Start(n) if is_z_down => {
                if stage == 1 && n + 1 >= STAGE1_START_LOG_SIZE {
                    State::Shoot
                } else {
                    State::Start(n + 1)
                }
            }
            State::End(n) if is_z_down => {
                if stage == 1 && n + 1 >= STAGE1_START_LOG_SIZE {
                    return (self, true);
                } else {
                    State::End(n + 1)
                }
            }
            _ => self,
        };
        (next, false)
    }
}
