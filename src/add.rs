use clap::{ArgMatches};
use mnemesis_utils::{Credentials, prompt_for_input, prompt_for_password, read_credentials, write_credentials};

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
    let path        = args.value_of("PATH").unwrap();
    let list        = read_credentials(path);
    let mut list    = list.iter().filter(|ref item| item.url != credentials.url && item.login != credentials.login).collect::<Vec<_>>();

    list.push(&credentials);
    write_credentials(path, list);
}
