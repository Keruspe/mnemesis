use clap::{ArgMatches};
use mnemesis_utils::{MnemesisUtils};
use mode::Mode;

use std::str::FromStr;

pub fn _main(args: &ArgMatches) {
    let path   = args.value_of("PATH").unwrap();
    let mode   = args.value_of("mode").ok_or(()).and_then(Mode::from_str).unwrap_or_else(|_| Mode::default());
    let utils  = MnemesisUtils::new();
    let entity = utils.read_entity(path);

    if let Some(entity) = entity {
        match mode {
            Mode::Raw => println!("{}", entity),
        }
    }
}
