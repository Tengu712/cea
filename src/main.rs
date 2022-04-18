pub mod winapis;

//use std::collections::HashMap;
use winapis::{direct3d::*, winapi::*, *};

/// A struct for application and resource bank to run the game.
struct Application {
    winapp: WindowsApplication,
    d3dapp: D3DApplication,
    //d2dapp: D2DApplication,
    //images: HashMap<ImgResID, Image>,
}

impl Application {
    /// Constructor.
    fn new() -> Result<Self, MyErr> {
        let winapp = WindowsApplication::new(
            "C:\\Users\\kazuki\\Documents\\programming\\rust\\cea\\res\\",
            "秘封俱楽部",
            1280,
            720,
            ask_yesno("Start with a fullscreen window?", "question"),
        )?;
        let d3dapp = D3DApplication::new(&winapp, 1280, 720)?;
        //let d2dapp = D2DApplication::new(&winapp).unwrap();
        //let mut images = HashMap::new();
        Ok(Self {
            winapp,
            d3dapp,
            //d2dapp,
            //images,
        })
    }
    /// **[Side Effect]**
    /// Run the game.
    fn run(self) -> Result<(), MyErr> {
        //let mut keystates = KeyStates::new();
        //let mut scene = title::TitleScene::new();
        while !self.winapp.do_event() {
            /*
            keystates = keystates.detect(KeyCode::Z).detect(KeyCode::L);
            let (next, reqs) = match scene {
                Scene::Title(n) => n.update(&keystates),
                Scene::Game(n) => n.update(&keystates, &self.dialogue),
            };
            scene = next;
            self.d2dapp.begin_draw();
            self.d2dapp.clear_screen(0.0, 0.0, 0.0);
            for req in reqs.get_array().iter() {
                self.do_request(req)?;
            }
            self.d2dapp.end_draw()?;
            self.d2dapp.present(1, 0)?;
            */
        }
        Ok(())
    }
    // /// **[Side Effect]**
    // /// Do requests of drawing image or.
    /*
    fn do_request(&self, request: &Request) -> Result<(), String> {
        match request {
            Request::Reverse(n) => self.d2dapp.reverse(n.clone(), 1280.0),
            Request::Image(n) => {
                let image = self
                    .images
                    .get(&n.key)
                    .ok_or(format!("{} : {:?}", "Invalid draw request.", &n.key))?;
                let width = n.width.unwrap_or(image.width as f32);
                let height = n.height.unwrap_or(image.height as f32);
                let uv_width = n.uv_width.unwrap_or(image.width as f32);
                let uv_height = n.uv_height.unwrap_or(image.height as f32);
                self.d2dapp.draw_image(
                    image, n.left, n.top, width, height, n.uv_left, n.uv_top, uv_width, uv_height,
                    n.alpha, n.center,
                );
            }
            Request::Text(n) => {
                let text = String::from_utf8_lossy(&n.text);
                self.d2dapp.draw_text(
                    text.into_owned().as_str(),
                    n.left,
                    n.top,
                    n.right,
                    n.bottom,
                    n.size,
                    n.alignment,
                    n.r,
                    n.g,
                    n.b,
                    n.a,
                )?;
            }
            _ => (),
        }
        Ok(())
    }
    */
}

/// Another entry point that's to return error to main function.
fn main_with_result() -> Result<(), MyErr> {
    Application::new()?.run()?;
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
