use crypto::digest::Digest;
use crypto::sha2::Sha512;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use json::JsonValue;
use rpassword::prompt_password_stdout;

use std::fmt;

use std::io::Read;
use std::fs::OpenOptions;

use std::collections::HashMap;
use reqwest::Url;
use reqwest::header::{USER_AGENT, CONTENT_TYPE};
use reqwest::Method;

const VERSION :&str = "1.7";
/// remote url, will be removed in future version
pub const URL: &str = "https://blih.epitech.eu";

/// Blih structure representing a Blih connection
///
/// Each intraction with Blih remote api is made with method
pub struct Blih {
    pub user_agent: String,
    pub url: String,
    pub user: Option<String>,
    pub token: Option<String>,
}

impl Blih {
    /// return a new Blith struct
    ///
    /// # Description
    ///
    /// If `user` is equal to `None`, the value of env var `BLIH_USER`
    /// If `token` is equal to `None`, the value of env var `BLIH_TOKEN`
    pub fn new(user: Option<&str>, token: Option<&str>, url: Option<&str>) -> Blih {
        let user = match user {
            Some(s) => Some(s.to_string()),
            None    => None,
        };
        let token = match token {
            Some(s) => Some(s.to_string()),
            None    => None,
        };
        let url = match url {
            Some(s) => String::from(s),
            None    => String::from("https://blih.epitech.eu"),
        };
        let user_agent = "blih-".to_owned() + VERSION;
        Blih {
            user_agent,
            url,
            user,
            token,
        }
    }

    /// Promt the user for the password, and store a `Sha512` of it.
    pub fn ask_password(&mut self) -> Result<(), BlihErr> {
        match prompt_password_stdout("Password: ") {
            Ok(s) => {
                let mut hash = Sha512::new();
                hash.input_str(&s);
                self.token = Some(hash.result_str());
            },
            Err(_) => self.token = None,
        };
        Ok(())
    }

    /// sign the data using `Hmac512` algorithm
    fn sign_token(&self, data: &Option<JsonValue>) -> Result<String, BlihErr> {
        let mut hmac = Hmac::new(Sha512::new(), match &self.token {
            Some(s) => s.as_bytes(),
            None    => return Err(BlihErr::NoTokenProvided),
        });
        hmac.input(match &self.user {
            Some(s) => s.as_bytes(),
            None    => return Err(BlihErr::NoUserNameProvided),
        });
        if data.is_some() {
            hmac.input(data.as_ref().unwrap().pretty(4).as_bytes());
        }
        Ok(hex::encode(hmac.result().code()))
    }

    /// send a whoami request.
    pub fn whoami(&self) -> Result<String, BlihErr> {
        self.request("/whoami", Method::GET, None)
    }

    /// list all the repo on the remote
    pub fn list_repo(&self) -> Result<String, BlihErr> {
        self.request("/repositories", Method::GET, None)
    }

    /// print info about the provided repo
    pub fn info_repo(&self, name: &str) -> Result<String, BlihErr> {
        self.request(&("/repository/".to_owned() + name), Method::GET, None)
    }

    /// delete the repository on the remote
    ///
    /// **WARNING** No confirmation required
    pub fn delete_repo(&self, name: &str) -> Result<String, BlihErr> {
        self.request(&("/repository/".to_owned() + name), Method::DELETE, None)
    }

    /// create a new repo
    pub fn create_repo(&self, name: &str) -> Result<String, BlihErr> {
        let mut map = HashMap::new();
        map.insert("name", name);
        map.insert("type", "git");
        self.request("/repositories", Method::POST, Some(JsonValue::from(map)))
    }

    pub fn get_acl(&self, name: &str) -> Result<String, BlihErr> {
        self.request(&("/repository/".to_owned() + name + "/acls"), Method::GET, None)
    }

    pub fn set_acl(&self, name: &str, user: &str, acl: &str) -> Result<String, BlihErr> {
        let mut map = HashMap::new();
        map.insert("acl", acl);
        map.insert("user", user);
        self.request(&("/repository/".to_owned() + name + "/acls"), Method::POST, Some(JsonValue::from(map)))
    }

    pub fn list_key(&self) -> Result<String, BlihErr> {
        self.request("/sshkeys", Method::GET, None)
    }

    pub fn upload_key_str(&self, key: &str) -> Result<String, BlihErr> {
        let mut map = HashMap::new();
        map.insert("sshkey", key);
        self.request("/sshkey", Method::POST, Some(JsonValue::from(map)))
    }

    pub fn upload_key_path(&self, key: &str) -> Result<String, BlihErr> {
        let mut file = match OpenOptions::new().read(true).open(key) {
            Ok(s)  => s,
            Err(_) => return Err(BlihErr::InvalidSshKey),
        };
        let mut key = String::new();
        match file.read_to_string(&mut key) {
            Ok(_)  => (),
            Err(_) => return Err(BlihErr::InvalidSshKey),
        }
        key = key.trim_matches('\n').to_string();
        self.upload_key_str(&key)
    }

    fn request(&self, path: &str, meth: Method, data: Option<JsonValue>) -> Result<String, BlihErr> {
        let mut map = JsonValue::new_object();
        let token = self.sign_token(&data);
        map.insert("user", match &self.user {
            Some(s) => s.as_str(),
            None    => return Err(BlihErr::NoUserNameProvided),
        }).unwrap();
        map.insert("signature", match &token {
            Ok(s)  => s.as_str(),
            Err(_) => return Err(BlihErr::NoTokenProvided),
        }).unwrap();
        if let Some(data) = data {
            map.insert("data", data).unwrap();
        }
        let uri = match Url::parse((self.url.clone() + path).as_str()) {
                Ok(o)  => o,
                Err(_) => return Err(BlihErr::InvalidUrl),
            };
        let client = reqwest::blocking::Client::new().request(meth, uri)
                    .header(USER_AGENT, &self.user_agent)
                    .header(CONTENT_TYPE, "application/json")
                    .body(map.dump()).send();
        let client = match client {
            Ok(o)  => o,
            Err(_) => return Err(BlihErr::RequestFailed),
        };
        Ok(client.text().unwrap())
    }
}

/// Blih error handling
#[derive(Debug, PartialEq)]
pub enum BlihErr {
    InvalidRequest,
    InvalidUrl,
    RequestFailed,
    NoTokenProvided,
    NoUserNameProvided,
    InvalidSshKey,
    HeaderError,
}

impl std::error::Error for BlihErr {}

impl fmt::Display for BlihErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BlihErr::InvalidRequest     => write!(f, "Invalid request"),
            BlihErr::InvalidUrl         => write!(f, "Invalid Url"),
            BlihErr::RequestFailed      => write!(f, "Request Failed"),
            BlihErr::NoTokenProvided    => write!(f, "No token was provided"),
            BlihErr::NoUserNameProvided => write!(f, "No username was provided"),
            BlihErr::InvalidSshKey      => write!(f, "Invalid sshkey file"),
            BlihErr::HeaderError        => write!(f, "Error while building header"),
        }
    }
}
