// this first import is the anyhow! macro, which allows you to throw errors
// explicitly the second import is anyhow's Context trait, a wrapper around
// Error that allows you to add context to it. That way, if something fails,
// it'll display that message. This trait also comes with the `with_context`
// function, allowing you to pass a closure returning something that impls
// Display instead of just something that impls Display. This could be useful
// when you want to include a certain variable in your context, like:
// result.with_context(|| format!("{}", variable)) or something.
use anyhow::{anyhow, Context as _};
use dirs::home_dir;

use std::{fs, path::PathBuf};

// anyhow::Result is the wrapper around it's Error enum type, it basically is
// Result<T, anyhow::Error> or something. Since a successful return will return
// a string, your Result should wrap a String
pub fn get_token() -> anyhow::Result<String> {
    // Create Path Buffer for potention home path (PathBufs aren't validated,
    // they're literally just wrappers around Strings)
    let cfg: PathBuf = home_dir()
        .map(|v| PathBuf::from(v).join(".wakatime.cfg"))
        .context("Failed to get home path")?;

    // Ensure path exists (these are methods on PathBuf)
    if !cfg.exists() || cfg.is_dir() {
        return Err(anyhow!("Wakatime config file not found"));
    }

    // Reading from path
    // read_to_string expects P: AsRef<Path>, which you can provide with just
    // &cfg, a reference to the PathBuf (which just wraps an OsString (which is
    // like a String, but can contain non-UTF8 characters))
    let cfg_content = fs::read_to_string(&cfg)
        .context("Failed to read from the config file")?;

    // Parsing the cfg lines for the token. You don't need an explicit type on
    // token here, string literals are automatically typed with &str and the
    // 'static lifetime
    let mut token = "";
    for line in cfg_content.lines() {
        if line.starts_with("api_key") {
            // You don't need an explicit type annotation when collecting, this
            // is implied (there's more logic behind this but it's not in the
            // scope of this explanation)
            let sects = line.split("=").collect::<Vec<_>>();

            // The .get() method will return an Option containing Some(item) if
            // that index is present in the Vec and None otherwise. Unwrapping
            // with context can give you the &str that you want.
            token = sects
                .get(1)
                .context("Line contained api_key but field was empty")?;

            break;
        }
    }

    // Since you're returning your final String in a Result, you'll need to wrap
    // it in Ok. The .into() method is implemented for a lot of commonly
    // switched-between types, and since you're returning a Result<String> and
    // there's an impl From<&str> for String, it can implicitly convert it. This
    // basically just calls .to_string() which just calls .to_owned()
    Ok(token.into())
}
