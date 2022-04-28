/// This is a logic of game.
mod gameapis;
///
mod resource;
/// This provides apis to call WindowsAPI.
mod winapis;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 960;

/// Entory point.
fn main() {
    match start_app() {
        Ok(_) => (),
        Err(e) => {
            let message = format!(
                "Error code : 0x{:X}\n{}",
                e.code().0,
                e.message().to_string_lossy()
            );
            println!("{}", message);
            winapis::winapi::show_messagebox(message, "Error")
        }
    }
}
/// Start application.
fn start_app() -> Result<(), windows::core::Error> {
    println!("Start the game.");
    let cur_dir = get_curdir_for_winapp().unwrap_or(String::from(r"\"));
    // Create window app
    println!("Create window.");
    let winapp = winapis::winapi::WindowsApplication::new(
        "",
        WIDTH as i32,
        HEIGHT as i32,
        winapis::winapi::ask_yesno("Start with a fullscreen window?", "question"),
    )?;
    // Create drawing app
    println!("Create Direct3D Components.");
    let d3dapp =
        winapis::direct3d::D3DApplication::new(&winapp, WIDTH, HEIGHT, cur_dir.clone().as_str())?;
    println!("Create DirectWrite Components.");
    let dwapp = d3dapp.create_text_module(&winapp)?;
    // Load
    println!("Start loading.");
    let config = resource::load_config(cur_dir.clone());
    let default_text_format = dwapp.create_text_format(" ", None, 64.0)?;
    let map_text_format = resource::load_font_collection(&dwapp, &config)?;
    let map_image = resource::load_images(&d3dapp, cur_dir.clone())?;
    let idea = create_idea(&d3dapp)?;
    // Run the app
    println!("Run the game.");
    let mut game = gameapis::Game::new();
    let mut cdata = create_default_cdata();
    let mut input = gameapis::component::Input::default();
    d3dapp.set_cdata(&cdata)?;
    while !winapp.do_event() {
        let mut reqs = Vec::with_capacity(gameapis::request::REQUESTS_SIZE);
        input.z = winapis::winapi::get_next_keystate(0x5A, input.z);
        input.x = winapis::winapi::get_next_keystate(0x58, input.x);
        input.s = winapis::winapi::get_next_keystate(0xA0, input.s);
        input.left = winapis::winapi::get_next_keystate(0x25, input.left);
        input.up = winapis::winapi::get_next_keystate(0x26, input.up);
        input.right = winapis::winapi::get_next_keystate(0x27, input.right);
        input.down = winapis::winapi::get_next_keystate(0x28, input.down);
        d3dapp.set_rtv();
        d3dapp.clear_rtv();
        let next = game.update(&mut reqs, &input);
        for i in reqs {
            match i {
                gameapis::request::Request::SetImage(n) => {
                    cdata = d3dapp.set_d3dimage(map_image.get(n.0), cdata)
                }
                gameapis::request::Request::UnsetImage => cdata = d3dapp.set_d3dimage(None, cdata),
                gameapis::request::Request::SetCData(n) => {
                    cdata = apply_cdata_diff(cdata, n);
                    d3dapp.set_cdata(&cdata)?;
                }
                gameapis::request::Request::SetView(n) => {
                    cdata.mat_view = winapis::math::Matrix4x4::new_view(n.pos, n.rot);
                    d3dapp.set_cdata(&cdata)?;
                }
                gameapis::request::Request::SetPerse(n) => {
                    cdata.mat_proj =
                        winapis::math::Matrix4x4::new_perse(n.w, n.h, n.theta, n.n, n.f);
                    d3dapp.set_cdata(&cdata)?;
                }
                gameapis::request::Request::SetOrtho(n) => {
                    cdata.mat_proj =
                        winapis::math::Matrix4x4::new_ortho(n.l, n.r, n.t, n.b, n.n, n.f);
                    d3dapp.set_cdata(&cdata)?;
                }
                gameapis::request::Request::Multiple => {
                    cdata.vec_prm[1] = 0.0;
                    d3dapp.set_cdata(&cdata)?;
                }
                gameapis::request::Request::Overlay => {
                    cdata.vec_prm[1] = 1.0;
                    d3dapp.set_cdata(&cdata)?;
                }
                gameapis::request::Request::DrawImage => d3dapp.draw_model(&idea),
                gameapis::request::Request::DrawText(n) => {
                    let desc = winapis::directwrite::TextDesc::new()
                        .set_text(n.text)
                        .set_rect(n.rect)
                        .set_rgba(n.rgba)
                        .set_align(n.align as u32);
                    dwapp.draw_text(
                        &desc,
                        map_text_format
                            .get(&n.format)
                            .unwrap_or(&default_text_format),
                    )?;
                }
            }
        }
        game = next;
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
fn create_idea(
    d3dapp: &winapis::direct3d::D3DApplication,
) -> Result<winapis::direct3d::model::ModelBuffer, windows::core::Error> {
    let data_vtx = [
        winapis::direct3d::model::Vertex {
            pos: [-0.5, -0.5, 0.0],
            col: [1.0; 4],
            tex: [0.0, 1.0],
        },
        winapis::direct3d::model::Vertex {
            pos: [-0.5, 0.5, 0.0],
            col: [1.0; 4],
            tex: [0.0, 0.0],
        },
        winapis::direct3d::model::Vertex {
            pos: [0.5, 0.5, 0.0],
            col: [1.0; 4],
            tex: [1.0, 0.0],
        },
        winapis::direct3d::model::Vertex {
            pos: [0.5, -0.5, 0.0],
            col: [1.0; 4],
            tex: [1.0, 1.0],
        },
    ];
    let data_idx = [0, 1, 2, 0, 2, 3];
    d3dapp.create_modelbuffer(4, &data_vtx, 6, &data_idx)
}
/// Create default constant buffer data.
fn create_default_cdata() -> winapis::direct3d::cbuffer::CData {
    winapis::direct3d::cbuffer::CData {
        mat_scl: winapis::math::Matrix4x4::new_identity(),
        mat_rtx: winapis::math::Matrix4x4::new_identity(),
        mat_rty: winapis::math::Matrix4x4::new_identity(),
        mat_rtz: winapis::math::Matrix4x4::new_identity(),
        mat_trs: winapis::math::Matrix4x4::new_identity(),
        mat_view: winapis::math::Matrix4x4::new_identity(),
        mat_proj: winapis::math::Matrix4x4::new_ortho(
            0.0,
            WIDTH as f32,
            HEIGHT as f32,
            0.0,
            0.0,
            1.0,
        ),
        vec_col: [1.0; 4],
        vec_prm: [0.0; 4],
    }
}
/// Apply constant buffer difference request to cdata.
fn apply_cdata_diff(
    cdata: winapis::direct3d::cbuffer::CData,
    cdata_diff: gameapis::request::cdata::CDataDiff,
) -> winapis::direct3d::cbuffer::CData {
    winapis::direct3d::cbuffer::CData {
        mat_scl: winapis::math::Matrix4x4::new_scaling(
            cdata_diff.scl_xy[0],
            cdata_diff.scl_xy[1],
            1.0,
        ),
        mat_rtx: winapis::math::Matrix4x4::new_rotation_x(cdata_diff.rot_xyz[0]),
        mat_rty: winapis::math::Matrix4x4::new_rotation_y(cdata_diff.rot_xyz[1]),
        mat_rtz: winapis::math::Matrix4x4::new_rotation_z(cdata_diff.rot_xyz[2]),
        mat_trs: winapis::math::Matrix4x4::new_translation(
            cdata_diff.trs_xyz[0],
            cdata_diff.trs_xyz[1],
            cdata_diff.trs_xyz[2],
        ),
        mat_view: cdata.mat_view,
        mat_proj: cdata.mat_proj,
        vec_col: cdata_diff.col_rgba,
        vec_prm: cdata.vec_prm,
    }
}
