use super::winapis::{
    direct3d::{
        model::{ModelBuffer, Vertex},
        D3DApplication,
    },
    winapi::*,
    *,
};

impl WErr {
    fn app(errknd: EKnd, message: &str) -> Self {
        WErr::from(errknd, String::from(message), String::from("App"))
    }
}

/// Start application.
/// Nothing to do for main user but handling WErr.
pub fn start_app() -> Result<(), WErr> {
    const WIDTH: u32 = 1280;
    const HEIGHT: u32 = 720;
    // Create window app
    let winapp = WindowsApplication::new(
        get_curdir_for_winapp()?,
        "秘封俱楽部",
        WIDTH as i32,
        HEIGHT as i32,
        ask_yesno("Start with a fullscreen window?", "question"),
    )?;
    // Create drawing app
    let d3dapp = D3DApplication::new(&winapp, WIDTH, HEIGHT)?;
    let d3dtxt = d3dapp.create_text_module(&winapp)?;
    // Create
    let idea = create_idea(&d3dapp)?;
    // Run the app
    while !winapp.do_event() {
        d3dapp.set_rtv();
        d3dapp.clear_rtv();
        d3dapp.swap()?;
    }
    Ok(())
}
/// Get current directory for WindowsApplication.
fn get_curdir_for_winapp() -> Result<String, WErr> {
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
    Ok(dir)
}
/// Create idea sprite.
fn create_idea(d3dapp: &D3DApplication) -> Result<ModelBuffer, WErr> {
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
    d3dapp.create_modelbuffer(4, &data_vtx, 6, &data_idx)
}
