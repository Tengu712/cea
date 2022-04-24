use super::gameapis::request::imgid::ImgID;
use super::winapis::{
    direct3d::{image::D3DImage, D3DApplication},
    WErr,
};
use std::collections::HashMap;

pub fn load_images(
    d3dapp: &D3DApplication,
    cur_dir: &String,
) -> Result<HashMap<ImgID, D3DImage>, WErr> {
    let mut map = HashMap::new();
    map.insert(
        ImgID::FlanB0,
        d3dapp.create_image_from_file(cur_dir.clone() + "ch_flan_b0.png")?,
    );
    map.insert(
        ImgID::FlanR0,
        d3dapp.create_image_from_file(cur_dir.clone() + "ch_flan_r0.png")?,
    );
    map.insert(
        ImgID::FlanL0,
        d3dapp.create_image_from_file(cur_dir.clone() + "ch_flan_l0.png")?,
    );
    map.insert(
        ImgID::RemiliaF0,
        d3dapp.create_image_from_file(cur_dir.clone() + "ch_remilia_f0.png")?,
    );
    map.insert(
        ImgID::UiFrame,
        d3dapp.create_image_from_file(cur_dir.clone() + "ui_frame.png")?,
    );
    map.insert(
        ImgID::StFlan,
        d3dapp.create_image_from_file(cur_dir.clone() + "st_flan0.png")?,
    );
    Ok(map)
}
