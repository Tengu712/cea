pub mod app;
pub mod function;

use super::winapis::*;

impl WErr {
    fn app(errknd: EKnd, message: &str) -> Self {
        WErr::from(errknd, String::from(message), String::from("App"))
    }
}

pub fn start_app() -> Result<(), WErr> {
    let cur_dir = std::env::current_dir()
        .map_err(|_| WErr::app(EKnd::Get, "current directory"))?
        .to_str()
        .ok_or(WErr::app(EKnd::Common, "Convertion curdir to str failed"))?
        .to_string()
        + "\\";
    let dir = std::env::args()
        .collect::<Vec<String>>()
        .get(1)
        .unwrap_or(&cur_dir)
        .clone();
    app::Application::new(dir)?.run()?;
    Ok(())
}
