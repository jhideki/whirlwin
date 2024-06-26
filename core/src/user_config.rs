use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub programs: Vec<String>,
    pub keybinds: HashMap<String, u32>,
}

impl Config {
    fn new() -> Self {
        let mut keybinds = HashMap::new();
        keybinds.insert("left".to_string(), 0);
        keybinds.insert("right".to_string(), 0);
        keybinds.insert("down".to_string(), 0);
        keybinds.insert("next".to_string(), 0);
        keybinds.insert("prev".to_string(), 0);
        keybinds.insert("above".to_string(), 0);
        let programs: Vec<String> = vec!["".to_string(); 10];
        Config { programs, keybinds }
    }
}

pub fn read_config() -> Result<Config, Box<dyn Error>> {
    let mut path = match get_path() {
        Ok(path) => path,
        Err(e) => return Err(e),
    };
    path.push("config.json");
    println!("{:?}", path);
    let mut file = match OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(&path)
    {
        Ok(file) => file,
        Err(e) => {
            return Err(format!("Error opening file {}", e).into());
        }
    };

    let mut json_data = String::new();
    if let Err(e) = file.read_to_string(&mut json_data) {
        return Err(format!("Failed to read user data from file {}", e).into());
    }

    let config = match serde_json::from_str(&json_data) {
        Ok(data) => data,
        Err(_) => Config::new(),
    };
    Ok(config)
}

fn get_path() -> Result<PathBuf, Box<dyn Error>> {
    let mut appdata_path = match env::var_os("LOCALAPPDATA") {
        Some(path) => PathBuf::from(path),
        None => {
            eprintln!("Failed to retreive APPDATA env var");
            return Err("Failed to retreive APPDATA env var".into());
        }
    };

    if !appdata_path.exists() {
        return Err(r"Appdata\Local does not exist".into());
    }
    appdata_path.push("whirlwin");
    if !appdata_path.exists() {
        if let Err(err) = std::fs::create_dir_all(&appdata_path) {
            return Err("Failed to create whirlwin dir".into());
        }
    }
    Ok(appdata_path)
}
