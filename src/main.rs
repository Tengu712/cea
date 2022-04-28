/// This is a logic of game.
mod gameapis;
///
mod resource;
/// This provides apis to call WindowsAPI.
mod winapis;

use gameapis::{asset::*, component::*, entity::*};
use std::collections::{BTreeMap, HashMap};
use winapis::{direct3d::*, winapi::*};

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
            show_messagebox(message, "Error")
        }
    }
}
/// Start application.
fn start_app() -> Result<(), windows::core::Error> {
    println!("\n==================================================");
    println!("            \"TITLE\"");
    println!("      SkyDog Assoc of WordSpiritism, Tengu712");
    println!("==================================================");
    println!("Starts up ...");
    let cur_dir = get_curdir_for_winapp().unwrap_or(String::from(r"\"));
    println!(" - Create a window");
    let winapp = WindowsApplication::new(
        "",
        WIDTH as i32,
        HEIGHT as i32,
        ask_yesno("Start with a fullscreen window?", "question"),
    )?;
    println!(" - Create Direct3D Components");
    let d3dapp = D3DApplication::new(&winapp, WIDTH, HEIGHT, cur_dir.clone().as_str())?;
    //println!(" - Create DirectWrite Components");
    //let dwapp = d3dapp.create_text_module(&winapp)?;
    println!(" - Load resources");
    let map_image = {
        let mut map = HashMap::new();
        let res_dir = cur_dir + r"img\";
        for i in IMGID_ARRAY {
            map.insert(i, d3dapp.create_image_from_file(res_dir.clone() + i)?);
        }
        map
    };
    println!(" - Create game components");
    let mut components = Components::default();
    create_player(&mut components);
    let idea = create_idea(&d3dapp)?;
    let mut cdata = create_default_cdata();
    d3dapp.set_cdata(&cdata)?;
    println!("\nAll clear. The game is starting.\n");
    while !winapp.do_event() {
        components.input.z = get_next_keystate(0x5A, components.input.z);
        components.input.x = get_next_keystate(0x58, components.input.x);
        components.input.s = get_next_keystate(0xA0, components.input.s);
        components.input.left = get_next_keystate(0x25, components.input.left);
        components.input.up = get_next_keystate(0x26, components.input.up);
        components.input.right = get_next_keystate(0x27, components.input.right);
        components.input.down = get_next_keystate(0x28, components.input.down);
        ComponentUpdater::process(&mut components.positions, &components.velocities);
        ComponentUpdater::process(&mut components.sprites, &components.positions);
        d3dapp.set_rtv();
        d3dapp.clear_rtv();
        let mut btmap = BTreeMap::new();
        for (_, v) in components.sprites.iter() {
            btmap.insert(v.layer, v);
        }
        for (_, v) in btmap.into_iter() {
            if let Some(imgid) = v.imgid {
                cdata = d3dapp.set_d3dimage(map_image.get(imgid), cdata);
            }
            cdata = apply_cdata_diff(cdata, v);
            d3dapp.set_cdata(&cdata)?;
            d3dapp.draw_model(&idea);
        }
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
fn create_idea(d3dapp: &D3DApplication) -> Result<model::ModelBuffer, windows::core::Error> {
    let data_vtx = [
        model::Vertex {
            pos: [-0.5, -0.5, 0.0],
            col: [1.0; 4],
            tex: [0.0, 1.0],
        },
        model::Vertex {
            pos: [-0.5, 0.5, 0.0],
            col: [1.0; 4],
            tex: [0.0, 0.0],
        },
        model::Vertex {
            pos: [0.5, 0.5, 0.0],
            col: [1.0; 4],
            tex: [1.0, 0.0],
        },
        model::Vertex {
            pos: [0.5, -0.5, 0.0],
            col: [1.0; 4],
            tex: [1.0, 1.0],
        },
    ];
    let data_idx = [0, 1, 2, 0, 2, 3];
    d3dapp.create_modelbuffer(4, &data_vtx, 6, &data_idx)
}
/// Create default constant buffer data.
fn create_default_cdata() -> cbuffer::CData {
    cbuffer::CData {
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
fn apply_cdata_diff(cdata: cbuffer::CData, sprite: &gameapis::component::Sprite) -> cbuffer::CData {
    cbuffer::CData {
        mat_scl: winapis::math::Matrix4x4::new_scaling(
            sprite.scaling.x,
            sprite.scaling.y,
            sprite.scaling.z,
        ),
        mat_rtx: winapis::math::Matrix4x4::new_rotation_x(sprite.rotation.x),
        mat_rty: winapis::math::Matrix4x4::new_rotation_x(sprite.rotation.y),
        mat_rtz: winapis::math::Matrix4x4::new_rotation_x(sprite.rotation.z),
        mat_trs: winapis::math::Matrix4x4::new_translation(
            sprite.translation.x,
            sprite.translation.y,
            sprite.translation.z,
        ),
        mat_view: cdata.mat_view,
        mat_proj: cdata.mat_proj,
        vec_col: [
            sprite.color.x,
            sprite.color.y,
            sprite.color.z,
            sprite.color.w,
        ],
        vec_prm: cdata.vec_prm,
    }
}
