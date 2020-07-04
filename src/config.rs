use json::JsonValue;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::PathBuf;

pub struct Config {
    pub user: Option<String>,
    pub token: Option<String>,
    pub baseurl: Option<String>,
    pub file: PathBuf,
    pub changed: bool,
}

impl Config {
    pub fn new_empty(path: Option<PathBuf>) -> Config {
        let path = match path {
            Some(s) => s,
            None => PathBuf::new(),
        };
        Config {
            user: None,
            token: None,
            baseurl: None,
            file: path,
            changed: false,
        }
    }

    pub fn new_json(content: String, file: PathBuf) -> Config {
        let mut parsed = match json::parse(&content) {
            Ok(o) => o,
            Err(e) => {
                eprintln!("Error with the json parser: {}", e);
                return Config::new_empty(Some(file));
            }
        };
        let user = parsed["user"].take_string();
        let token = parsed["token"].take_string();
        let baseurl = parsed["baseurl"].take_string();
        Config {
            user: user,
            token: token,
            baseurl: baseurl,
            file: file.clone(),
            changed: false,
        }
    }

    pub fn get_config(args: Option<&str>) -> Config {
        let config_path = get_config_path(args);
        let mut file = match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&config_path)
        {
            Ok(o) => o,
            Err(_) => return Config::new_empty(Some(config_path)),
        };
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        if content.is_empty() {
            Config::new_empty(Some(config_path))
        } else {
            Config::new_json(content, config_path)
        }
    }

    pub fn dump(&self) -> Result<Box<JsonValue>, ()> {
        let mut content = Box::new(JsonValue::new_object());
        if self.user.is_some() {
            match content.insert("user", self.user.as_ref().unwrap().as_str()) {
                Err(_) => return Err(()),
                _ => (),
            }
        }
        if self.token.is_some() {
            match content.insert("token", self.token.as_ref().unwrap().as_str()) {
                Err(_) => return Err(()),
                _ => (),
            }
        }
        if self.baseurl.is_some() {
            match content.insert("baseurl", self.baseurl.as_ref().unwrap().as_str()) {
                Err(_) => return Err(()),
                _ => (),
            }
        }
        Ok(content)
    }

    pub fn write_config(&self) {
        let json = self.dump().unwrap().pretty(4);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.file)
            .unwrap();
        file.write_all(json.as_bytes()).unwrap();
        file.set_len(json.as_bytes().len() as u64).unwrap();
    }
}

impl Default for Config {
    fn default() -> Config {
        Config::new_empty(None)
    }
}

/// return the path to the config folder
///
/// If the `BLIH_PATH` env variable is set, use it's value
///
/// else if the `-p` is set, use it's value
///
/// else use `$HOME/.config/blih`
fn get_config_path(args: Option<&str>) -> PathBuf {
    let home_path = std::env::var("HOME");
    let env_path = std::env::var("BLIH_PATH");

    let mut val = match env_path {
        Ok(s) => s,
        Err(_) => match args {
            Some(s) => String::from(s),
            None => match home_path {
                Ok(s) => s,
                Err(_) => String::from("."),
            },
        },
    };
    val.push_str("/.config/blih");
    PathBuf::from(val)
}
