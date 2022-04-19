pub mod winapis;

use winapis::{
    direct3d::{cbuffer::*, model::*, text::*, *},
    math::*,
    winapi::*,
    *,
};

impl MyErr {
    fn app(errknd: EKnd, message: &str) -> Self {
        Self {
            message: String::from(message),
            kind: errknd_string(errknd),
            place: String::from("App"),
        }
    }
}

/// A struct for application and resource bank to run the game.
struct Application {
    winapp: WindowsApplication,
    d3dapp: D3DApplication,
    d3dtxt: D3DTextModule,
    idea: ModelBuffer,
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
        let d3dtxt = d3dapp.create_text_module(&winapp)?;
        let idea = {
            let data_vtx = [
                Vertex {
                    pos: [-0.5, -0.5, 0.0],
                    col: [1.0; 4],
                    tex: [0.0, 1.0],
                },
                Vertex {
                    pos: [-0.5, 0.5, 0.0],
                    col: [1.0; 4],
                    tex: [0.0, 0.0],
                },
                Vertex {
                    pos: [0.5, 0.5, 0.0],
                    col: [1.0; 4],
                    tex: [1.0, 0.0],
                },
                Vertex {
                    pos: [0.5, -0.5, 0.0],
                    col: [1.0; 4],
                    tex: [1.0, 1.0],
                },
            ];
            let data_idx = [0, 1, 2, 0, 2, 3];
            d3dapp.create_modelbuffer(4, &data_vtx, 6, &data_idx)?
        };
        Ok(Self {
            winapp,
            d3dapp,
            d3dtxt,
            idea,
        })
    }
    /// **[Side Effect]**
    /// Run the game.
    fn run(self) -> Result<(), MyErr> {
        let mut cdata = CData {
            mat_scl: Matrix4x4::new_scaling(640.0, 640.0, 1.0),
            mat_rtx: Matrix4x4::new_identity(),
            mat_rty: Matrix4x4::new_identity(),
            mat_rtz: Matrix4x4::new_identity(),
            mat_trs: Matrix4x4::new_translation(0.0, 0.0, 0.0),
            mat_view: Matrix4x4::new_identity(),
            mat_proj: Matrix4x4::new_ortho(-640.0, 640.0, 360.0, -360.0, 0.0, 1.0),
            vec_col: [1.0; 4],
            vec_prm: [0.0; 4],
        };
        let image = self
            .d3dapp
            .create_image_from_file(r"C:\Users\kazuki\OneDrive\touhou\sozai\th_abp\bg_title.png")?;
        let text_desc = DrawingTextDesc {
            text: String::from("秘封俱楽部"),
            font: String::from("メイリオ"),
            size: 64.0,
            rect: [0.0, 1280.0, 0.0, 720.0],
            rgba: [1.0; 4],
            align: TextAlign::Left,
        };
        while !self.winapp.do_event() {
            self.d3dapp.set_rtv();
            self.d3dapp.clear_rtv();
            cdata = self.d3dapp.set_d3dimage(Some(&image), cdata);
            self.d3dapp.set_cdata(&cdata)?;
            self.d3dapp.draw_model(&self.idea)?;
            self.d3dtxt.draw_text(&text_desc)?;
            self.d3dapp.swap()?;
        }
        Ok(())
    }
}

/// Another entry point that's to return error to main function.
fn main_with_result() -> Result<(), MyErr> {
    let current_dir = std::env::current_dir()
        .map_err(|_| MyErr::app(EKnd::Get, "current directory"))?
        .to_str()
        .ok_or(MyErr::app(EKnd::Common, "Convertion curdir to str failed"))?
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
            show_messagebox(e.get_message(), e.get_title());
        }
    }
}
