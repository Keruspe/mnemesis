use clap::{ArgMatches};
use mnemesis_utils::{Credentials, Entity, prompt_for_input, prompt_for_password, read_entities, write_entities};

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
    let mut list    = read_entities(path);
    let concurrent  = list.iter().position(|entity| match *entity {
        Entity::Credentials(ref creds) => creds.url == credentials.url && creds.login == credentials.login,
        //_                              => false,
    });

    list.push(Entity::Credentials(credentials));

    if let Some(concurrent) = concurrent {
        list.swap_remove(concurrent);
    }

    write_entities(path, list);
}
