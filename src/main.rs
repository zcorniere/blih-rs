extern crate blih_rs;
extern crate reqwest;
extern crate json;
#[macro_use]extern crate clap;

use blih_rs::Blih;
use blih_rs::BlihErr;
use clap::App;
use clap::ArgMatches;

fn main()
{
    let yaml = load_yaml!("cli.yml");
    let args = App::from_yaml(yaml).get_matches();

    let mut auth = Blih::new(args.value_of("user"), args.value_of("token"));
    if auth.get_user().is_none() == true {
        println!("{}", BlihErr::NoUserNameProvided);
        return;
    }
    if auth.get_token().is_none() == true {
        if auth.ask_password().is_err() {
            println!("{}", BlihErr::NoUserNameProvided);
        }
    }

    let res = match args.subcommand_name() {
        Some("whoami")     => auth.whoami(),
        Some("repository") => repo_sub_cmd(args.subcommand_matches("repository"), &auth),
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
