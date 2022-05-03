/// This is a logic of game.
mod gameapis;
/// This provides apis to call WindowsAPI.
mod winapis;

use gameapis::{asset::*, component::*, scene::*, *};
use std::collections::HashMap;
use winapis::{direct3d::*, directwrite::*, math::*, winapi::*};

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
    set_console_mode_vtp(&get_std_handle())?;
    println!("\n==================================================");
    println!("            \"TITLE\"");
    println!("      SkyDog Assoc of WordSpiritism, Tengu712");
    println!("==================================================\n");
    println!("Starts up ...");
    const WIDTH: u32 = 1280;
    const HEIGHT: u32 = 960;
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
            print!("\r\x1b[2K    * {}", i);
            map.insert(i, d3dapp.create_image_from_file(res_dir.clone() + i)?);
        }
        map
    };
    println!("\r\x1b[2K - Set up runtime objects");
    print!("\r\x1b[2K    * square polygon");
    let idea = create_idea(&d3dapp)?;
    print!("\r\x1b[2K    * constant buffer data");
    let mut cdata = create_default_cdata(WIDTH as f32, HEIGHT as f32);
    d3dapp.set_cdata(&cdata)?;
    print!("\r\x1b[2K    * camera matrixes");
    let mat_view = Matrix4x4::new_view([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
    let mat_proj = Matrix4x4::new_ortho(0.0, WIDTH as f32, HEIGHT as f32, 0.0, 0.0, 1000.0);
    let mat_proj_3d = Matrix4x4::new_perse(
        WIDTH as f32,
        HEIGHT as f32,
        45.0f32.to_radians(),
        1.0,
        1000.0,
    );
    print!("\r\x1b[2K    * input data");
    let mut input = Input::default();
    print!("\r\x1b[2K    * game coms");
    let mut world = World::default();
    let mut scene = Title::new(&mut world);
    println!("\r\x1b[2K\nAll clear. The game is starting.\n");
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
                world.emngr.camera.pos.x,
                world.emngr.camera.pos.y,
                world.emngr.camera.pos.z,
            ],
            [
                world.emngr.camera.rot.x,
                world.emngr.camera.rot.y,
                world.emngr.camera.rot.z,
            ],
        );
        cdata.mat_proj = mat_proj_3d.clone();
        for (_, s, v) in world.emngr.coms.sprite3ds.iter() {
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
        // Draw 2d objects
        d3dapp.set_rtv(false);
        cdata.mat_view = mat_view.clone();
        cdata.mat_proj = mat_proj.clone();
        let mut vec_2d =
            Vec::with_capacity(world.emngr.coms.sprites.len() + world.emngr.coms.texts.len());
        for (k, s, v) in world.emngr.coms.sprites.iter() {
            if !s.is_active() {
                continue;
            }
            vec_2d.push((v.translation.z, Some(k), None));
        }
        for (k, s, v) in world.emngr.coms.texts.iter() {
            if !s.is_active() {
                continue;
            }
            vec_2d.push((v.layer, None, Some(k)));
        }
        vec_2d.sort_by(|(z1, _, _), (z2, _, _)| z1.partial_cmp(z2).unwrap());
        for (_, sprite, text) in vec_2d {
            if let Some(sprite_id) = sprite {
                if let Some(v) = world.emngr.coms.sprites.get(sprite_id) {
                    match v.imgid {
                        Some(imgid) => cdata = d3dapp.set_d3dimage(map_image.get(imgid), cdata),
                        None => cdata = d3dapp.set_d3dimage(None, cdata),
                    }
                    cdata = apply_cdata_diff(cdata, v);
                    d3dapp.set_cdata(&cdata)?;
                    d3dapp.draw_model(&idea);
                }
            } else if let Some(text_id) = text {
                if let Some(v) = world.emngr.coms.texts.get(text_id) {
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
            }
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
        vec_prm: [
            cdata.vec_prm[0],
            sprite.mode,
            cdata.vec_prm[2],
            cdata.vec_prm[3],
        ],
    }
}
