/// This is a logic of game.
mod gameapis;
/// This provides apis to call WindowsAPI.
mod winapis;

use gameapis::{asset::*, component::*, scene::*, *};
use std::collections::HashMap;
use winapis::{direct3d::*, directwrite::*, math::*, winapi::*};

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
    println!("    * camera matrixes");
    let mat_view = Matrix4x4::new_view([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
    let mat_proj = Matrix4x4::new_ortho(0.0, WIDTH as f32, HEIGHT as f32, 0.0, 0.0, 1000.0);
    let mat_proj_3d = Matrix4x4::new_perse(
        WIDTH as f32,
        HEIGHT as f32,
        45.0f32.to_radians(),
        1.0,
        1000.0,
    );
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
        if let Some(next) = scene.as_mut().update(&mut world) {
            scene = next;
        }
        world.update(&input);
        d3dapp.clear_rtv();
        // Draw 3d sprite
        d3dapp.set_rtv(true);
        cdata.mat_view = Matrix4x4::new_view(
            [
                world.manager.camera.pos.x,
                world.manager.camera.pos.y,
                world.manager.camera.pos.z,
            ],
            [
                world.manager.camera.rot.x,
                world.manager.camera.rot.y,
                world.manager.camera.rot.z,
            ],
        );
        cdata.mat_proj = mat_proj_3d.clone();
        for (_, s, v) in world.manager.components.sprite3ds.iter() {
            if !s.is_active() {
                continue;
            }
            match v.imgid {
                Some(imgid) => cdata = d3dapp.set_d3dimage(map_image.get(imgid), cdata),
                None => cdata = d3dapp.set_d3dimage(None, cdata),
            }
            cdata = apply_cdata_diff(cdata, v);
            d3dapp.set_cdata(&cdata)?;
            d3dapp.draw_model(&idea);
        }
        // Draw 2d sprite
        d3dapp.set_rtv(false);
        cdata.mat_view = mat_view.clone();
        cdata.mat_proj = mat_proj.clone();
        let mut sprites_t = Vec::with_capacity(world.manager.components.sprites.len());
        for (k, s, v) in world.manager.components.sprites.iter() {
            if !s.is_active() {
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
        // Draw 2d text
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
        mat_scl: Matrix4x4::new_identity(),
        mat_rtx: Matrix4x4::new_identity(),
        mat_rty: Matrix4x4::new_identity(),
        mat_rtz: Matrix4x4::new_identity(),
        mat_trs: Matrix4x4::new_identity(),
        mat_view: Matrix4x4::new_identity(),
        mat_proj: Matrix4x4::new_ortho(0.0, WIDTH as f32, HEIGHT as f32, 0.0, 0.0, 1000.0),
        vec_col: [1.0; 4],
        vec_prm: [0.0; 4],
    }
}
/// Apply constant buffer difference request to cdata.
fn apply_cdata_diff(cdata: cbuffer::CData, sprite: &gameapis::component::Sprite) -> cbuffer::CData {
    cbuffer::CData {
        mat_scl: Matrix4x4::new_scaling(sprite.scaling.x, sprite.scaling.y, sprite.scaling.z),
        mat_rtx: Matrix4x4::new_rotation_x(sprite.rotation.x),
        mat_rty: Matrix4x4::new_rotation_y(sprite.rotation.y),
        mat_rtz: Matrix4x4::new_rotation_z(sprite.rotation.z),
        mat_trs: Matrix4x4::new_translation(
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
