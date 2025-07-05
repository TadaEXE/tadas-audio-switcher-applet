use home;
use serde::{Deserialize, Serialize};
use std::{fs::{self, File}, path::PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub slots: Vec<Slot>,
    pub id_counter: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Sink {
    pub name: String,
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Slot {
    pub id: u32,
    pub sink: Sink,
}

pub fn get_config_path() -> PathBuf {
    let mut home_dir = home::home_dir().unwrap();
    home_dir.push(".tada");
    home_dir.push("tasa_conf.toml");
    home_dir
}

fn config_exists_or_create() {
    let path = get_config_path();
    if !path.exists() {
        fs::create_dir_all(&path.parent().unwrap()).unwrap();
        File::create(&path).unwrap();
        let fresh_config = Config{
            slots: vec![],
            id_counter: 0,
        };
        write_config(&fresh_config);
        println!("Created fresh config file at {:#?}", &path);
    }
}

pub fn load_config() -> Config {
    config_exists_or_create();
    let content = fs::read_to_string(get_config_path()).unwrap();
    let config = toml::from_str(&content).unwrap();
    config
}

pub fn write_config(config: &Config) {
    config_exists_or_create();
    let content = toml::to_string_pretty(config).unwrap();
    let _ = fs::write(get_config_path(), content);
}
