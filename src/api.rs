use anyhow::{anyhow, Context as _};
use dirs::home_dir;

use std::{fs, path::PathBuf};

pub fn get_token() -> anyhow::Result<String> {
    let cfg: PathBuf = home_dir()
        .map(|v| PathBuf::from(v).join(".wakatime.cfg"))
        .context("Failed to get home path")?;

    if !cfg.exists() || cfg.is_dir() {
        return Err(anyhow!("Wakatime config file not found"));
    }

    let cfg_content = fs::read_to_string(&cfg).context("Failed to read from the config file")?;

    let mut token = "";
    for line in cfg_content.lines() {
        if line.starts_with("api_key") {
            let sects = line.split("=").collect::<Vec<_>>();
            token = sects
                .get(1)
                .context("Line contained api_key but field was empty")?;
            break;
        }
    }

    Ok(token.into())
}
