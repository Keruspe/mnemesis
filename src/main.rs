extern crate clap;
extern crate mnemesis_utils;

mod action;
mod add;
mod get;
mod mode;

use clap::{App, Arg, SubCommand};

use action::Action;

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
                         .required(true))
                    .arg(Arg::with_name("mode")
                         .short("m")
                         .long("mode")
                         .takes_value(true)
                         .possible_values(&["raw"])
                         .default_value("raw")))
        .arg(Arg::with_name("PATH"))
        .arg(Arg::with_name("mode")
             .short("m")
             .long("mode")
             .takes_value(true)
             .possible_values(&["raw"])
             .default_value("raw")
             .requires("PATH"))
        .get_matches();
    let action  = Action::from_matches(matches);

    match action {
        Ok(Action::Add(g)) => add::_main(g),
        Ok(Action::Get(g)) => get::_main(g),
        Err(e)             => eprintln!("error: {}", e),
    }
}
