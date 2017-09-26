extern crate clap;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

mod data;

use clap::{App,Arg,SubCommand};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
                        .version(env!("CARGO_PKG_VERSION"))
                        .author(env!("CARGO_PKG_AUTHORS"))
                        .about(env!("CARGO_PKG_DESCRIPTION"))
                        .get_matches();
}
