use clap::{ArgMatches};
use mnemesis_utils::{prompt_for_input, prompt_for_password};
use serde_json::{self};

use data::Credentials;

pub fn _main(args: &ArgMatches) {
    let url         = prompt_for_input("Url: ");
    let login       = prompt_for_input("Login: ");
    let password    = prompt_for_password("Password: ");
    let totp_secret = if args.is_present("totp") {
        Some(prompt_for_input("TOTP secret: "))
    } else {
        None
    };
    let credentials = Credentials {
        url,
        login,
        password,
        totp_secret,
    };

    println!("{:?}", serde_json::to_string(&credentials));
}
