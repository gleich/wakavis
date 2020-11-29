use dirs::home_dir;
use std::path::PathBuf;

pub fn get_token() {
    // Locating and ensuring that the config file exists
    let cfg: PathBuf = match home_dir() {
        Some(path) => PathBuf::from(path).join(".wakatime.cfg"),
        None => PathBuf::from(""),
    };
    if !cfg.exists() {
        panic!("Wakatime config file not found");
    }
}
