use super::gameapis::request::{imgid::*, text::TextFormat};
use super::winapis::{
    direct3d::{image::D3DImage, D3DApplication},
    directwrite::DWriteApp,
};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};
use windows::Win32::Graphics::DirectWrite::IDWriteTextFormat;

pub struct Config {
    numbers_font: String,
    dialogue_font: String,
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
        let mut res = line_splited[1].to_string();
        res.push('\0');
        if line_splited[0] == "NumbersFont" {
            config.numbers_font = res;
        } else if line_splited[0] == "DialogueFont" {
            config.dialogue_font = res;
        }
    }
    config
}

pub fn load_font_collection(
    dwapp: &DWriteApp,
    config: &Config,
) -> Result<HashMap<TextFormat, IDWriteTextFormat>, windows::core::Error> {
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
    map.insert(
        TextFormat::Graze,
        dwapp.create_text_format(config.numbers_font.as_str(), None, 56.0)?,
    );
    map.insert(
        TextFormat::GameOver,
        dwapp.create_text_format(config.dialogue_font.as_str(), None, 56.0)?,
    );
    Ok(map)
}

pub fn load_images(
    d3dapp: &D3DApplication,
    cur_dir: String,
) -> Result<HashMap<&str, D3DImage>, windows::core::Error> {
    let mut map = HashMap::new();
    let res_dir = cur_dir + r"img\";
    for i in IMGID_ARRAY {
        map.insert(i.0, d3dapp.create_image_from_file(res_dir.clone() + i.0)?);
    }
    Ok(map)
}
