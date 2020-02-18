use lib_blih::Blih;
use json::JsonValue;
use http::{Request, Response};

fn main()
{
    let mut auth = Blih::new("/whoami".to_string(), Some("zacharie.corniere@epitech.eu".to_string()), None);
    auth.ask_password();

    let mut json = JsonValue::new_object();
    json.insert("user", auth.get_user().as_str());
    json.insert("signature", auth.get_token().as_ref().unwrap().as_str());
    let request = Request::get(lib_blih::URL.to_owned() + auth.get_request())
                      .header("User-Agent", auth.get_user_agent())
                      .header("Content-Type", "application/json")
                      .body(json.dump()).unwrap();
}
