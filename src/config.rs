use ratatui::style::Color;
use serde::{de::DeserializeOwned, Deserialize};

use std::{
    fs, io,
    path::{Path, PathBuf},
};

trait Config {}

// TODO lots of code duplication
#[derive(Debug, Deserialize)]
struct ConfigTomlUser {
    colours: Option<ColoursUser>,
}
impl Config for ConfigTomlUser {}

#[derive(Debug, Deserialize)]
struct ColoursUser {
    normal: Option<Color>,
    info: Option<Color>,
    search: Option<Color>,
    command: Option<Color>,
    text: Option<Color>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigToml {
    pub colours: Colours,
}
impl Config for ConfigToml {}

#[derive(Debug, Deserialize)]
pub struct Colours {
    pub normal: Color,
    pub info: Color,
    pub search: Color,
    pub command: Color,
    pub text: Color,
}

// Used for development.
const RELATIVE_CONFIG_STR: &str = r"./config.toml";
// Default.
const DEFAULT_CONFIG_STR: &str = r".config/pacbrow/default-config.toml";
// User-defined settings. Can overwrite.
const CONFIG_STR: &str = r".config/pacbrow/config.toml";

pub fn read_config() -> io::Result<ConfigToml> {
    let home_dir = match home::home_dir() {
        Some(path) if !path.as_os_str().is_empty() => path,
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Unable to locate home dir.",
            ))
        }
    };

    let mut default_config_buf = PathBuf::from(&home_dir);
    default_config_buf.push(DEFAULT_CONFIG_STR);

    let mut config_buf = PathBuf::from(&home_dir);
    config_buf.push(CONFIG_STR);

    fn read_toml<T: Config + DeserializeOwned>(path: &Path) -> io::Result<T> {
        println!("{}", path.display());
        match fs::read_to_string(path) {
            Ok(toml_str) => match toml::from_str(&toml_str) {
                Ok(config_toml) => Ok(config_toml),
                Err(e) => Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("{} ({})", e, path.display()),
                )),
            },
            Err(e) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("{} ({})", e, path.display()),
            )),
        }
    }

    let mut config_toml =
        if let Ok(config_toml) = read_toml::<ConfigToml>(Path::new(RELATIVE_CONFIG_STR)) {
            config_toml
        } else {
            read_toml::<ConfigToml>(&default_config_buf)?
        };

    // Update config_toml with user settings
    if let Ok(user_conf) = read_toml::<ConfigTomlUser>(&config_buf) {
        if let Some(colours) = user_conf.colours {
            if let Some(normal) = colours.normal {
                config_toml.colours.normal = normal;
            }
            if let Some(info) = colours.info {
                config_toml.colours.info = info;
            }
            if let Some(search) = colours.search {
                config_toml.colours.search = search;
            }
            if let Some(command) = colours.command {
                config_toml.colours.command = command;
            }
            if let Some(text) = colours.text {
                config_toml.colours.text = text;
            }
        }
    };

    Ok(config_toml)
}
