extern crate blih_rs;
extern crate reqwest;
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
        Err(e) => panic!("{}", e),
    };
    println!("{}", res);
}

fn repo_sub_cmd(args: Option<&ArgMatches>, auth: &Blih) -> Result<String, BlihErr>
{
    let args = match args {
        Some(s) => s,
        None    => panic!(),
    };
    match args.subcommand_matches("list") {
        Some(_) => auth.list_repo(),
        None    => Ok(String::from("Not a valid sub command of \"repository\"")),
    }
}
