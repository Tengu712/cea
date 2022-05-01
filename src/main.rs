/// This is a logic of game.
mod gameapis;
/// This provides apis to call WindowsAPI.
mod winapis;

use gameapis::{asset::*, component::*, scene::*, system::*, *};
use std::collections::HashMap;
use winapis::{direct3d::*, directwrite::*, winapi::*};

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 960;

/// Entory point.
fn main() {
    match start_app() {
        Ok(_) => (),
        Err(e) => {
            let message = format!(
                "code : 0x{:X}\n{}",
                e.code().0,
                e.message().to_string_lossy()
            );
            println!("\nError! The game will not start.");
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
    println!("==================================================\n");
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
    println!(" - Create DirectWrite Components");
    let dwapp = d3dapp.create_text_module(&winapp)?;
    println!(" - Load resources");
    let map_image = {
        let mut map = HashMap::new();
        let res_dir = cur_dir + r"img\";
        for i in IMGID_ARRAY {
            println!("    * {}", i);
            map.insert(i, d3dapp.create_image_from_file(res_dir.clone() + i)?);
        }
        map
    };
    println!(" - Set up runtime objects");
    println!("    * square polygon");
    let idea = create_idea(&d3dapp)?;
    println!("    * constant buffer data");
    let mut cdata = create_default_cdata();
    d3dapp.set_cdata(&cdata)?;
    println!("    * input data");
    let mut input = Input::default();
    println!("    * game components");
    let mut world = World::default();
    let mut scene = Title::new(&mut world);
    println!("\nAll clear. The game is starting.\n");
    while !winapp.do_event() {
        input.z = get_next_keystate(0x5A, input.z);
        input.x = get_next_keystate(0x58, input.x);
        input.s = get_next_keystate(0xA0, input.s);
        input.left = get_next_keystate(0x25, input.left);
        input.up = get_next_keystate(0x26, input.up);
        input.right = get_next_keystate(0x27, input.right);
        input.down = get_next_keystate(0x28, input.down);
        world.update(&input);
        if let Some(next) = scene.as_mut().update(&mut world) {
            scene = next;
        }
        d3dapp.clear_rtv();
        d3dapp.set_rtv(false);
        let mut sprites_t = Vec::with_capacity(world.manager.components.sprites.len());
        for (k, s, v) in world.manager.components.sprites.iter() {
            if !s.is_active() {
                continue;
            }
            if !v.visible {
                continue;
            }
            sprites_t.push((v.translation.z, k));
        }
        sprites_t.sort_by(|(z1, _), (z2, _)| z1.partial_cmp(z2).unwrap());
        for (_, k) in sprites_t {
            if let Some(v) = world.manager.components.sprites.get(k) {
                match v.imgid {
                    Some(imgid) => cdata = d3dapp.set_d3dimage(map_image.get(imgid), cdata),
                    None => cdata = d3dapp.set_d3dimage(None, cdata),
                }
                cdata = apply_cdata_diff(cdata, v);
                d3dapp.set_cdata(&cdata)?;
                d3dapp.draw_model(&idea);
            }
        }
        for (_, s, v) in world.manager.components.texts.iter() {
            if !s.is_active() {
                continue;
            }
            if !v.visible {
                continue;
            }
            let desc = TextDesc {
                text: v.text.clone(),
                rect: [v.rect.l, v.rect.r, v.rect.t, v.rect.b],
                rgba: [v.rgba.x, v.rgba.y, v.rgba.z, v.rgba.w],
            };
            let formatdesc = TextFormatDesc {
                fontname: v.fontname,
                size: v.size,
                align: match v.align {
                    TextAlign::Left => 0,
                    TextAlign::Center => 1,
                    TextAlign::Right => 2,
                },
            };
            dwapp.draw_text(&desc, &formatdesc, None)?;
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
            1000.0,
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
        mat_rty: winapis::math::Matrix4x4::new_rotation_y(sprite.rotation.y),
        mat_rtz: winapis::math::Matrix4x4::new_rotation_z(sprite.rotation.z),
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
