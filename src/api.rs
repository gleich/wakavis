use dirs::home_dir;
use std::fs;
use std::path::PathBuf;

pub fn get_token() -> String {
    // Locating and ensuring that the config file exists
    let cfg: PathBuf = home_dir()
        .map(|v| PathBuf::from(v).join(".wakatime.cfg"))
        .expect("Failed to get home path");
    if !cfg.exists() || cfg.is_dir() {
        panic!("Wakatime config file not found");
    }
    // Reading from the config file
    let path = cfg.into_os_string().into_string().unwrap();
    let cfg_content = fs::read_to_string(path).expect("Failed to read from the config file");
    // Parsing the cfg lines for the token
    let mut token: &str = "";
    for line in cfg_content.lines() {
        if line.starts_with("api_key") {
            let sects = line.split("=").collect::<Vec<&str>>();
            if sects.len() == 0 {
                panic!("Found line with api token but no token supplied")
            }
            token = sects[1];
            break;
        }
    }
    if token == "" {
        panic!("Failed to get config file")
    }
    token.to_string()
}
