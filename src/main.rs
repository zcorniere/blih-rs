use lib_blih::Blih;
use std::collections::HashMap;

use reqwest::Url;
use reqwest::header::USER_AGENT;

fn main()
{
    let mut auth = Blih::new("/whoami".to_string(), Some("zacharie.corniere@epitech.eu".to_string()), None);
    auth.ask_password();

    let mut map = HashMap::new();
    map.insert("user", auth.get_user().as_str());
    map.insert("body", auth.get_token().as_ref().unwrap().as_str());

    let mut uri = String::from(lib_blih::URL);
    uri.push_str(auth.get_request());
    let uri = Url::parse(uri.as_str()).unwrap();
    let client = reqwest::blocking::Client::new();
    let res = client.get(uri)
                 .header(USER_AGENT, auth.get_user_agent())
                 .json(&map).send();

    let res = match res {
        Ok(o) => o,
        Err(e) => {
            println!("Error : {}", e);
            panic!();
        },
    };
    println!("res");
    println!("status = {:?}", res.status());
    println!("body = {:?}", res.text().unwrap());
}
