use crypto::digest::Digest;
use crypto::sha2::Sha512;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use rpassword::read_password;

static VERSION :&str = "1.7";

pub struct Blih {
    url: String,
    user: String,
    token: Option<String>,
}

impl Blih {
    pub fn new(url :String, user :Option<String>, token :Option<String>) -> Blih {
        let user = match user {
            Some(s) => s,
            None => std::env::var("BLIH_USER").unwrap(),
        };
        Blih {
            url,
            user,
            token,
        }
    }

    pub fn ask_password(&mut self) {
        match read_password() {
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
}
