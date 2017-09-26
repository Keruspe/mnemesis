extern crate clap;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

mod add;

use clap::{App, SubCommand};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(SubCommand::with_name("add").about("Add new credentials to the registry"))
        .get_matches();

    if let Some(sub) = matches.subcommand_matches("add") {
        add::_main(sub);
    } else {
        println!("{}", matches.usage());
    }
}
