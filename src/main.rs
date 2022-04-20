pub mod gameapis;
/// This provides apis to call WindowsAPI.
pub mod winapis;

use gameapis::Game;
use winapis::{direct3d::D3DApplication, directwrite::text::*, winapi::*, *};

/// Entory point.
fn main() {
    match start_app() {
        Ok(_) => (),
        Err(e) => winapis::winapi::show_messagebox(e.get_message(), e.get_title()),
    }
}
/// Start application.
pub fn start_app() -> Result<(), WErr> {
    const WIDTH: u32 = 1280;
    const HEIGHT: u32 = 720;
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
    //
    let collection = dwapp.create_custom_font(cur_dir + "SatsukiGendaiMincho-M.ttf")?;
    let text = TextDesc::new().set_text("コンピュータの世界が");
    let format = dwapp.create_text_format("さつき源代明朝", collection, 64.0)?;
    // Run the app
    while !winapp.do_event() {
        d3dapp.set_rtv();
        d3dapp.clear_rtv();
        dwapp.draw_text(&text, &format)?;
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
