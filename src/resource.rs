use super::gameapis::request::{imgid::ImgID, text::TextFormat};
use super::winapis::{
    direct3d::{image::D3DImage, D3DApplication},
    directwrite::DWriteApp,
    WErr,
};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};
use windows::Win32::Graphics::DirectWrite::IDWriteTextFormat;

pub struct Config {
    pub numbers_font: String,
    pub dialogue_font: String,
}
impl Config {
    fn new() -> Self {
        Self {
            numbers_font: String::from(" "),
            dialogue_font: String::from(" "),
        }
    }
}
pub fn load_config(cur_dir: String) -> Config {
    let mut config = Config::new();
    let file = match File::open(cur_dir + "config.cfg") {
        Ok(n) => n,
        Err(_) => return config,
    };
    let reader = BufReader::new(file);
    for i in reader.lines() {
        let line = match i {
            Ok(n) => n,
            Err(_) => return config,
        };
        let line_splited = line.split('=').collect::<Vec<&str>>();
        if line_splited.len() != 2 {
            continue;
        }
        if line_splited[0] == "NumbersFont" {
            config.numbers_font = line_splited[1].to_string();
        } else if line_splited[0] == "DialogueFont" {
            config.dialogue_font = line_splited[1].to_string();
        }
    }
    config
}
pub fn load_font_collection(
    dwapp: &DWriteApp,
    config: &Config,
) -> Result<HashMap<TextFormat, IDWriteTextFormat>, WErr> {
    let mut map = HashMap::new();
    map.insert(
        TextFormat::Normal,
        dwapp.create_text_format(config.dialogue_font.as_str(), None, 42.0)?,
    );
    map.insert(
        TextFormat::Fps,
        dwapp.create_text_format(config.numbers_font.as_str(), None, 32.0)?,
    );
    map.insert(
        TextFormat::Score,
        dwapp.create_text_format(config.numbers_font.as_str(), None, 60.0)?,
    );
    Ok(map)
}

pub fn load_images(
    d3dapp: &D3DApplication,
    cur_dir: String,
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