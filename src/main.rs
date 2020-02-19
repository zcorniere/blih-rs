use lib_blih::Blih;

fn main()
{
    let mut auth = Blih::new("/whoami".to_string(), Some("zacharie.corniere@epitech.eu".to_string()), None, reqwest::Method::GET);
    auth.ask_password();

    let res = match auth.send_request() {
        Ok(o) => o,
        Err(e) => panic!("{}", e),
    };
    println!("res");
    println!("status = {:?}", res.status());
    println!("body = {:?}", res.text().unwrap());
}
