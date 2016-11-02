extern crate clap;
extern crate rustc_serialize;

mod data;

use clap::{App,Arg,SubCommand};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
                        .version(env!("CARGO_PKG_VERSION"))
                        .author(env!("CARGO_PKG_AUTHORS"))
                        .about(env!("CARGO_PKG_DESCRIPTION"))
                        .arg(Arg::with_name("version")
                                    .short("v")
                                    .long("version")
                                    .help("Prints version information"))
                        .get_matches();

    if matches.is_present("version") {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    }
}
