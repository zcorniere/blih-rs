use crypto::digest::Digest;
use crypto::sha2::Sha512;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use rpassword::prompt_password_stdout;

use std::fmt;

use std::collections::HashMap;
use reqwest::Url;
use reqwest::header::USER_AGENT;
use reqwest::Method;

const VERSION :&str = "1.7";
pub const URL: &str = "https://blih.epitech.eu";

pub struct Blih {
    user_agent: String,
    user: Option<String>,
    token: Option<String>,
}

impl Blih {
    /// return a new Blith struct
    pub fn new(user :Option<&str>, token :Option<&str>) -> Blih {
        let user = match user {
            Some(s) => Some(String::from(s)),
            None    => match std::env::var("BLIH_USER") {
                    Ok(o)  => Some(o),
                    Err(_) => None,
                },
        };
        let token = match token {
            Some(s) => Some(String::from(s)),
            None    => match std::env::var("BLIH_TOKEN") {
                    Ok(o) => Some(o),
                    Err(_)  => None,
                },
        };
        let user_agent = String::from("blih-".to_owned() + VERSION);
        Blih {
            user_agent,
            user,
            token,
        }
    }

    /// Promt the user for his password.
    pub fn ask_password(&mut self) -> Result<(), BlihErr> {
        match prompt_password_stdout("Password: ") {
            Ok(s) => {
                let mut hash = Sha512::new();
                hash.input_str(&s);
                let hex = hash.result_str();
                let mut hmac = Hmac::new(Sha512::new(), hex.as_bytes());
                hmac.input(match &self.user {
                    Some(s) => s.as_bytes(),
                    None    => return Err(BlihErr::NoUserNameProvided),
                });
                let hex = hmac.result();
                let hex = hex.code();
                self.token = Some(hex::encode(hex))
            },
            Err(_) => self.token = None,
        };
        Ok(())
    }

    /// return the user_agent
    pub fn get_user_agent(&self) -> &String {
        &self.user_agent
    }

    /// return the user
    pub fn get_user(&self) -> &Option<String> {
        &self.user
    }

    /// return the token
    pub fn get_token(&self) -> &Option<String> {
        &self.token
    }

    pub fn whoami(&self) -> Result<String, BlihErr> {
        self.request("/whoami", Method::GET, None)
    }

    pub fn list_repo(&self) -> Result<String, BlihErr> {
        self.request("/repositories", Method::GET, None)
    }

    pub fn info_repo(&self, name: &str) -> Result<String, BlihErr> {
        self.request(&("/repository/".to_owned() + name), Method::GET, None)
    }

    pub fn delete_repo(&self, name: &str) -> Result<String, BlihErr> {
        self.request(&("/repository/".to_owned() + name), Method::DELETE, None)
    }

    pub fn create_repo(&self, name: &str) -> Result<String, BlihErr> {
        let mut map = HashMap::new();
        map.insert("name", name);
        map.insert("type", "git");
        self.request("/repositories", Method::POST, Some(map))
    }

    fn request(&self, path: &str, meth: Method, map_sup: Option<HashMap<&str, &str>>) -> Result<String, BlihErr> {
        let mut map = HashMap::new();
        map.insert("user", match &self.user {
            Some(s) => s.as_str(),
            None    => return Err(BlihErr::NoUserNameProvided),
        });
        map.insert("signature", match &self.token {
            Some(s) => s.as_str(),
            None    => return Err(BlihErr::NoTokenProvided),
        });
        if map_sup.is_some() {
            for (k, v) in map_sup.unwrap().drain() {
                map.insert(k, v);
            }
        }
        let mut uri = String::from(URL);
        uri.push_str(path);
        let uri = match Url::parse(uri.as_str()) {
                Ok(o)  => o,
                Err(_) => return Err(BlihErr::InvalidUrl),
            };

        let client = reqwest::blocking::Client::new();
        let res = client.request(meth, uri)
                    .header(USER_AGENT, &self.user_agent)
                    .json(&map).send();
        let res = match res {
            Ok(o)  => o,
            Err(_) => return Err(BlihErr::RequestFailed),
        };
        Ok(res.text().unwrap())
    }
}

#[derive(Debug, PartialEq)]
pub enum BlihErr {
    InvalidRequest,
    InvalidUrl,
    RequestFailed,
    NoTokenProvided,
    NoUserNameProvided,
}

impl fmt::Display for BlihErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BlihErr::InvalidRequest     => write!(f, "Invalid request"),
            BlihErr::InvalidUrl         => write!(f, "Invalid Url"),
            BlihErr::RequestFailed      => write!(f, "Request Failed"),
            BlihErr::NoTokenProvided    => write!(f, "No token was provided"),
            BlihErr::NoUserNameProvided => write!(f, "No username was provided"),
        }
    }
}
