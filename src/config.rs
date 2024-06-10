use ratatui::style::Color;
use serde::{de::DeserializeOwned, Deserialize};

use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::app::Mode;

trait Config {}

// TODO too much code duplication
#[derive(Debug, Deserialize)]
struct ConfigTomlUser {
    colours: Option<ColoursUser>,
    operation: Option<OperationUser>,
}
impl Config for ConfigTomlUser {}

#[derive(Debug, Deserialize)]
struct ColoursUser {
    normal: Option<Color>,
    info: Option<Color>,
    search: Option<Color>,
    command: Option<Color>,
    display: Option<Color>,
    text: Option<Color>,
}

#[derive(Debug, Deserialize)]
pub struct OperationUser {
    pub starting_mode: Option<Mode>,
    pub allow_colon_in_search: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigToml {
    pub colours: Colours,
    pub operation: Operation,
}
impl Config for ConfigToml {}

#[derive(Debug, Deserialize)]
pub struct Colours {
    pub normal: Color,
    pub info: Color,
    pub search: Color,
    pub command: Color,
    pub display: Color,
    pub text: Color,
}

#[derive(Debug, Deserialize)]
pub struct Operation {
    pub starting_mode: Mode,
    pub allow_colon_in_search: bool,
}

// Used for development.
const DEV_CONFIG_STR: &str = "./config.toml";
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

    fn read_toml<T: Config + DeserializeOwned + std::fmt::Debug>(path: &Path) -> io::Result<T> {
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

    // Will not overwrite config if running debug target
    let allow_user_overwrite: bool;

    let mut config_toml =
        if let Ok(config_toml) = read_toml::<ConfigToml>(Path::new(DEV_CONFIG_STR)) {
            println!("ping");
            allow_user_overwrite = false;
            config_toml
        } else {
            allow_user_overwrite = true;
            read_toml::<ConfigToml>(&default_config_buf)?
        };

    // Update config_toml with user settings
    if allow_user_overwrite {
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
                if let Some(display) = colours.display {
                    config_toml.colours.display = display;
                }
                if let Some(text) = colours.text {
                    config_toml.colours.text = text;
                }
            }

            if let Some(operation) = user_conf.operation {
                if let Some(starting_mode) = operation.starting_mode {
                    config_toml.operation.starting_mode = starting_mode;
                }
                if let Some(allow_colon_in_search) = operation.allow_colon_in_search {
                    config_toml.operation.allow_colon_in_search = allow_colon_in_search;
                }
            }
        };
    }

    Ok(config_toml)
}
