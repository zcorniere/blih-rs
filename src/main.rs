extern crate blih_rs;
extern crate json;
#[macro_use]extern crate clap;

#[allow(dead_code)]mod config;
use config::Config;

use blih_rs::Blih;
use blih_rs::BlihErr;
use clap::App;
use clap::ArgMatches;

fn main()
{
    let yaml = load_yaml!("cli.yml");
    let args = App::from_yaml(yaml).get_matches();

    #[cfg(feature = "config")]
    let mut config = Config::get_config(args.value_of("path"));
    #[cfg(not(feature = "config"))]
    let mut config = Config::new_empty(None);

    let mut auth = blih_from_config(&args, &mut config);
    if auth.get_user().is_none() == true {
        println!("{}", BlihErr::NoUserNameProvided);
        return;
    }
    if auth.get_token().is_none() == true {
        if auth.ask_password().is_err() {
            println!("{}", BlihErr::NoUserNameProvided);
        } else {
            #[cfg(feature = "config")]
            config.set_token(auth.get_token());
        }
    }

    let res = match args.subcommand_name() {
        Some("whoami")     => auth.whoami(),
        Some("repository") => repo_sub_cmd(args.subcommand_matches("repository"), &auth),
        Some("sshkey")     => sshkey_sub_cmd(args.subcommand_matches("sshkey"), &auth),
        _                  => {
            println!("No command provided. Rerun with -h");
            return;
            },
    };
    let res = match res {
        Ok(o) => o,
        Err(e) => panic!("Err: {}", e),
    };
    let res = match json::parse(&res) {
        Ok(s)  => s,
        Err(s) => panic!("Malformed request : {}", s),
    };
    println!("{}", res.pretty(4));
    #[cfg(feature = "config")]
    config.write_config();
}

fn blih_from_config(args: &ArgMatches, config: &mut Config) -> Blih
{
    let user = match args.value_of("user") {
        Some(s) => Some(s.to_string()),
        None    => match std::env::var("BLIH_USER") {
            Ok(o)  => Some(o),
            Err(_) => match config.get_user() {
                Some(s) => Some(s.to_string()),
                None    => None,
            },
        },
    };
    let token = match args.value_of("token") {
        Some(s) => Some(s.to_string()),
        None    => match std::env::var("BLIH_TOKEN") {
            Ok(o)  => Some(o),
            Err(_) => match config.get_token() {
                Some(s) => Some(s.to_string()),
                None    => None,
            },
        },
    };
    let baseurl = match args.value_of("baseurl") {
        Some(s) => Some(s.to_string()),
        None    => match std::env::var("BLIH_URL") {
            Ok(o)  => Some(o),
            Err(_) => match config.get_baseurl() {
                Some(s) => Some(s.to_string()),
                None    => None,
            },
        },
    };
    config.set_user(&user);
    config.set_token(&token);
    config.set_baseurl(&baseurl);
    Blih::new(
        user.as_deref(), token.as_deref(), baseurl.as_deref())
}

fn repo_sub_cmd(args: Option<&ArgMatches>, auth: &Blih) -> Result<String, BlihErr>
{
    let args = match args {
        Some(s) => s,
        None    => panic!(),
    };
    match args.subcommand_name() {
        Some("list") => auth.list_repo(),
        Some("info") => auth.info_repo(args.subcommand_matches("info").unwrap().value_of("NAME").unwrap()),
        Some("delete") => auth.delete_repo(args.subcommand_matches("delete").unwrap().value_of("NAME").unwrap()),
        Some("create") => auth.create_repo(args.subcommand_matches("create").unwrap().value_of("NAME").unwrap()),
        Some("getacl") => auth.get_acl(args.subcommand_matches("getacl").unwrap().value_of("NAME").unwrap()),
        Some("setacl") => auth.set_acl(args.subcommand_matches("setacl").unwrap().value_of("NAME").unwrap(), args.subcommand_matches("setacl").unwrap().value_of("USER").unwrap(),args.subcommand_matches("setacl").unwrap().value_of("ACL").unwrap()),
        None | Some(_) => panic!(),
    }
}

fn sshkey_sub_cmd(args: Option<&ArgMatches>, auth: &Blih) -> Result<String, BlihErr>
{
    let args = match args {
        Some(s) => s,
        None    => panic!(),
    };
    match args.subcommand_name() {
        Some("list")   => auth.list_key(),
        Some("upload") => auth.upload_key_path(args.subcommand_matches("upload").unwrap().value_of("PATH").unwrap()),
        None | Some(_) => panic!(),
    }
}
