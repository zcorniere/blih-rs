use crypto::digest::Digest;
use crypto::sha2::Sha512;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use rpassword::prompt_password_stdout;

mod blih_err;
use crate::blih_err::BlihErr;

use std::collections::HashMap;
use reqwest::Url;
use reqwest::blocking::Response;
use reqwest::header::USER_AGENT;

const VERSION :&str = "1.7";
pub const URL: &str = "https://blih.epitech.eu";

pub struct Blih {
    user_agent: String,
    method: reqwest::Method,
    request: String,
    user: String,
    token: Option<String>,
}

impl Blih {
    /// return a new Blith struct
    pub fn new(request :String, user :Option<String>, token :Option<String>, method :reqwest::Method) -> Blih {
        let user = match user {
            Some(s) => s,
            None    => std::env::var("BLIH_USER").unwrap(),
        };
        let token = match token {
            Some(s) => Some(s),
            None    => match std::env::var("BLIH_TOKEN") {
                    Ok(val) => Some(val),
                    Err(_)  => None,
                },
        };
        let user_agent = String::from("blih-".to_owned() + VERSION);
        Blih {
            user_agent,
            method,
            request,
            user,
            token,
        }
    }

    // /*TODO*/ get password from env var
    /// Promt the user for his password.
    pub fn ask_password(&mut self) {
        match prompt_password_stdout("Password: ") {
            Ok(s) => {
                let mut hash = Sha512::new();
                hash.input_str(&s);
                let hex = hash.result_str();
                let mut hmac = Hmac::new(Sha512::new(), hex.as_bytes());
                hmac.input(self.user.as_bytes());
                let hex = hmac.result();
                let hex = hex.code();
                self.token = Some(hex::encode(hex))
            },
            Err(_) => self.token = None,
        };
    }

    /// return the user_agent
    pub fn get_user_agent(&self) -> &String {
        &self.user_agent
    }

    /// return the action to be executed
    pub fn get_request(&self) -> &String {
        &self.request
    }

    /// return the user
    pub fn get_user(&self) -> &String {
        &self.user
    }

    /// return the token
    pub fn get_token(&self) -> &Option<String> {
        &self.token
    }

    pub fn send_request(&self) -> Result<Response, BlihErr> {
        let mut map = HashMap::new();
        map.insert("user", self.user.as_str());
        map.insert("signature", match &self.token {
            Some(s) => s.as_str(),
            None    => return Err(BlihErr::NoTokenProvided),
        });
        let mut uri = String::from(URL);
        uri.push_str(&self.request);
        let uri = match Url::parse(uri.as_str()) {
                Ok(o)  => o,
                Err(_) => return Err(BlihErr::InvalidUrl),
            };

        let client = reqwest::blocking::Client::new();
        let res = client.request(self.method.clone(), uri)
                    .header(USER_AGENT, &self.user_agent)
                    .json(&map).send();
        let res = match res {
            Ok(o)  => o,
            Err(_) => return Err(BlihErr::RequestFailed),
        };
        Ok(res)
    }
}
