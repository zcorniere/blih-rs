use rpassword::read_password;

static VERSION :&str = "1.7";

pub struct Blih {
    url :String,
    user :String,
    token :Option<String>,
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
            Ok(s) => self.token = Some(s),
            Err(_) => self.token = None,
        };
    }
}
