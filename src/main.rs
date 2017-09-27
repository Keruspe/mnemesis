extern crate clap;
extern crate mnemesis_utils;

mod add;
mod get;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(SubCommand::with_name("add")
                    .about("Add new credentials to the registry")
                    .arg(Arg::with_name("PATH")
                         .required(true))
                    .arg(Arg::with_name("totp")
                         .short("t")
                         .long("totp")))
        .subcommand(SubCommand::with_name("get")
                    .about("Get some credentials from the registry")
                    .arg(Arg::with_name("PATH")
                         .required(true)))
        .arg(Arg::with_name("PATH"))
        .get_matches();

    if let Some(sub) = matches.subcommand_matches("add") {
        add::_main(sub);
    } else if let Some(sub) = matches.subcommand_matches("get") {
        get::_main(sub);
    } else if matches.is_present("PATH") {
        get::_main(&matches);
    } else {
        println!("{}", matches.usage());
    }
}
