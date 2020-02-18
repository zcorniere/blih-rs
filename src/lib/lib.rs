use crypto::digest::Digest;
use crypto::sha2::Sha512;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use rpassword::prompt_password_stdout;

const VERSION :&str = "1.7";
const URL: &str = "https://blih.epitech.eu/";

pub struct Blih {
    user_agent: String,
    request: String,
    user: String,
    token: Option<String>,
}

impl Blih {
    /// return a new Blith struct
    pub fn new(request :String, user :Option<String>, token :Option<String>) -> Blih {
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
                hash.input_str(&self.user);
                let hex = hash.result_str();
                let mut hmac = Hmac::new(Sha512::new(), hex.as_bytes());
                hmac.input(s.as_bytes());
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
}
