extern crate blih_rs;
extern crate json;
#[macro_use]
extern crate clap;

#[allow(dead_code)]
mod config;
use config::Config;

use blih_rs::Blih;
use blih_rs::BlihErr;
use clap::App;
use clap::ArgMatches;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let args = App::from_yaml(yaml).get_matches();

    #[cfg(feature = "config")]
    let mut config = Config::get_config(args.value_of("path"));
    #[cfg(not(feature = "config"))]
    let mut config = Config::default();

    let mut auth = blih_from_config(&args, &mut config);
    if auth.user.is_none() == true {
        println!("{}", BlihErr::NoUserNameProvided);
        return;
    }
    if auth.token.is_none() {
        if auth.ask_password().is_err() {
            println!("{}", BlihErr::NoUserNameProvided);
        } else {
            config.token = auth.token.clone();
        }
    }

    let res = match args.subcommand() {
        ("whoami", _) => auth.whoami(),
        ("repository", Some(s)) => repo_sub_cmd(s, &auth),
        ("sshkey", Some(s)) => sshkey_sub_cmd(s, &auth),
        _ => {
            println!("No command provided. Rerun with -h");
            return;
        }
    };
    let res = match res {
        Ok(o) => o,
        Err(e) => panic!("Err: {}", e),
    };
    let res = match json::parse(&res) {
        Ok(s) => s,
        Err(s) => panic!("Malformed request : {}", s),
    };
    println!("{}", res.pretty(4));
    #[cfg(feature = "config")]
    config.write_config();
}

fn blih_from_config(args: &ArgMatches, config: &mut Config) -> Blih {
    let user = match args.value_of("user") {
        Some(s) => Some(s.to_string()),
        None => match std::env::var("BLIH_USER") {
            Ok(o) => Some(o),
            Err(_) => match &config.user {
                Some(s) => Some(s.to_string()),
                None => None,
            },
        },
    };
    let token = match args.value_of("token") {
        Some(s) => Some(s.to_string()),
        None => match std::env::var("BLIH_TOKEN") {
            Ok(o) => Some(o),
            Err(_) => match &config.token {
                Some(s) => Some(s.to_string()),
                None => None,
            },
        },
    };
    let baseurl = match args.value_of("baseurl") {
        Some(s) => Some(s.to_string()),
        None => match std::env::var("BLIH_URL") {
            Ok(o) => Some(o),
            Err(_) => match &config.baseurl {
                Some(s) => Some(s.to_string()),
                None => None,
            },
        },
    };
    config.user = user;
    config.token = token;
    config.baseurl = baseurl;
    Blih::new(config.user.as_deref(), config.token.as_deref(), config.baseurl.as_deref())
}

fn repo_sub_cmd(args: &ArgMatches, auth: &Blih) -> Result<String, BlihErr> {
    match args.subcommand() {
        ("list", Some(_)) => auth.list_repo(),
        ("info", Some(s)) => auth.info_repo(s.value_of("NAME").unwrap()),
        ("delete", Some(s)) => auth.delete_repo(s.value_of("NAME").unwrap()),
        ("create", Some(s)) => auth.create_repo(s.value_of("NAME").unwrap()),
        ("getacl", Some(s)) => auth.get_acl(s.value_of("NAME").unwrap()),
        ("setacl", Some(s)) => auth.set_acl(
            s.value_of("NAME").unwrap(),
            s.value_of("USER").unwrap(),
            s.value_of("ACL").unwrap(),
        ),
        (_, _) => panic!(),
    }
}

fn sshkey_sub_cmd(args: &ArgMatches, auth: &Blih) -> Result<String, BlihErr> {
    match args.subcommand() {
        ("list", Some(_)) => auth.list_key(),
        ("upload", Some(s)) => auth.upload_key_path(s.value_of("PATH").unwrap()),
        (_, _) => panic!(),
    }
}
