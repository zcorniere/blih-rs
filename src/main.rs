use lib_blih::Blih;

fn main()
{
    let mut auth = Blih::new("/whoami".to_string(), Some("popopo".to_string()), None);
    auth.ask_password();
}
