/// This is a logic of game.
pub mod gameapis;
/// This provides apis to call WindowsAPI.
pub mod winapis;

use gameapis::Game;
use winapis::{
    direct3d::{
        cbuffer::CData,
        model::{ModelBuffer, Vertex},
        D3DApplication,
    },
    math::Matrix4x4,
    winapi::*,
    *,
};

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

/// Entory point.
fn main() {
    match start_app() {
        Ok(_) => (),
        Err(e) => winapis::winapi::show_messagebox(e.get_message(), e.get_title()),
    }
}
/// Start application.
pub fn start_app() -> Result<(), WErr> {
    let cur_dir = get_curdir_for_winapp().unwrap_or(String::from(r"\"));
    // Create window app
    let winapp = WindowsApplication::new(
        cur_dir.clone(),
        "秘封俱楽部",
        WIDTH as i32,
        HEIGHT as i32,
        ask_yesno("Start with a fullscreen window?", "question"),
    )?;
    // Create drawing app
    let d3dapp = D3DApplication::new(&winapp, WIDTH, HEIGHT)?;
    let dwapp = d3dapp.create_text_module(&winapp)?;
    // Load
    let fontcollection = dwapp.create_custom_font(cur_dir + "SatsukiGendaiMincho-M.ttf");
    // Run the app
    let idea = create_idea(&d3dapp)?;
    let mut cdata = create_default_cdata();
    let mut game = Game::new();
    d3dapp.set_cdata(&cdata);
    while !winapp.do_event() {
        d3dapp.set_rtv();
        d3dapp.clear_rtv();
        game = game.update();
        d3dapp.swap()?;
    }
    Ok(())
}
/// Get current directory for WindowsApplication.
fn get_curdir_for_winapp() -> Result<String, ()> {
    let cur_dir = std::env::current_dir()
        .map_err(|_| ())?
        .to_str()
        .ok_or(())?
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
/// Create default constant buffer data.
fn create_default_cdata() -> CData {
    CData {
        mat_scl: Matrix4x4::new_identity(),
        mat_rtx: Matrix4x4::new_identity(),
        mat_rty: Matrix4x4::new_identity(),
        mat_rtz: Matrix4x4::new_identity(),
        mat_trs: Matrix4x4::new_identity(),
        mat_view: Matrix4x4::new_identity(),
        mat_proj: Matrix4x4::new_ortho(0.0, WIDTH as f32, 0.0, HEIGHT as f32, 0.0, 100.0),
        vec_col: [1.0; 4],
        vec_prm: [0.0; 4],
    }
}
