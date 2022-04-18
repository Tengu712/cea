pub mod winapis;

//use std::collections::HashMap;
use winapis::{direct3d::*, winapi::*, *};

/// A struct for application and resource bank to run the game.
struct Application {
    winapp: WindowsApplication,
    d3dapp: D3DApplication,
}

impl Application {
    /// Constructor.
    fn new(dir: String) -> Result<Self, MyErr> {
        let winapp = WindowsApplication::new(
            dir,
            "秘封俱楽部",
            1280,
            720,
            ask_yesno("Start with a fullscreen window?", "question"),
        )?;
        let d3dapp = D3DApplication::new(&winapp, 1280, 720)?;
        Ok(Self { winapp, d3dapp })
    }
    /// **[Side Effect]**
    /// Run the game.
    fn run(self) -> Result<(), MyErr> {
        while !self.winapp.do_event() {
            self.d3dapp.set_rtv();
            self.d3dapp.clear_rtv();
            self.d3dapp.swap()?;
        }
        Ok(())
    }
}

/// Another entry point that's to return error to main function.
fn main_with_result() -> Result<(), MyErr> {
    let current_dir = std::env::current_dir()
        .map_err(|_| MyErr::App(ErrKnd::Get, String::from("current directory")))?
        .to_str()
        .ok_or(MyErr::App(
            ErrKnd::Common,
            String::from("Convertion current directory to str"),
        ))?
        .to_string()
        + "\\";
    let dir = std::env::args()
        .collect::<Vec<String>>()
        .get(1)
        .unwrap_or(&current_dir)
        .clone();
    Application::new(dir)?.run()?;
    Ok(())
}

/// Entry point. Unwrap application error here.
fn main() {
    match main_with_result() {
        Ok(()) => (),
        Err(e) => {
            let (message, title) = myerr_msg_ttl(e);
            show_messagebox(message, title);
        }
    }
}
