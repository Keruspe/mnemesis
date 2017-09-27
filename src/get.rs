use clap::{ArgMatches};
use mnemesis_utils::{MnemesisUtils};

pub fn _main(args: &ArgMatches) {
    let path  = args.value_of("PATH").unwrap();
    let utils = MnemesisUtils::new();
    let list  = utils.read_entities(path);

    println!("{:#?}", list);
}
