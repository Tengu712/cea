/// This is a logic of game.
mod gameapis;
/// This provides apis to call WindowsAPI.
mod winapis;

use gameapis::{asset::*, component::*, entity::*, system::*, *};
use std::collections::{BTreeMap, HashMap};
use winapis::{direct3d::*, directwrite::*, winapi::*};

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
    println!(" - Create DirectWrite Components");
    let dwapp = d3dapp.create_text_module(&winapp)?;
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
    let mut world = World::default();
    create_player(&mut world.manager);
    create_player_slow(&mut world.manager);
    create_enemy(&mut world.manager);
    create_frame(&mut world.manager);
    create_fps(&mut world.manager);
    world.systems.push(system_fpsmeasure_text);
    world.systems.push(system_input_velocity_player);
    world.systems.push(system_velocity_position);
    world.systems.push(system_restrict_position);
    world.systems.push(system_sameposition);
    world.systems.push(system_position_sprite);
    world.systems.push(system_playeranimation_sprite);
    println!(" - Set up drawing objects");
    let idea = create_idea(&d3dapp)?;
    let mut cdata = create_default_cdata();
    let mut input = Input::default();
    d3dapp.set_cdata(&cdata)?;
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
        d3dapp.set_rtv();
        d3dapp.clear_rtv();
        let mut btmap_sprite = BTreeMap::new();
        for v in world.manager.components.sprites.values() {
            btmap_sprite.insert(v.layer, v);
        }
        let mut btmap_text = BTreeMap::new();
        for v in world.manager.components.texts.values() {
            btmap_text.insert(v.layer, v);
        }
        for v in btmap_sprite.values() {
            match v.imgid {
                Some(imgid) => cdata = d3dapp.set_d3dimage(map_image.get(imgid), cdata),
                None => cdata = d3dapp.set_d3dimage(None, cdata),
            }
            cdata = apply_cdata_diff(cdata, v);
            d3dapp.set_cdata(&cdata)?;
            d3dapp.draw_model(&idea);
        }
        for v in btmap_text.values() {
            let desc = TextDesc {
                text: v.text.clone(),
                rect: [v.rect.l, v.rect.r, v.rect.t, v.rect.b],
                rgba: [v.rgba.x, v.rgba.y, v.rgba.z, v.rgba.w],
            };
            let formatdesc = TextFormatDesc {
                fontname: v.fontname,
                size: v.size,
                align: match v.align {
                    TextAlign2::Left => 0,
                    TextAlign2::Center => 1,
                    TextAlign2::Right => 2,
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
