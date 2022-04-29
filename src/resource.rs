use std::{
    fs::File,
    io::{BufRead, BufReader},
};

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
