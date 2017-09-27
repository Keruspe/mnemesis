use clap::{ArgMatches};
use mnemesis_utils::{Credentials, Entity, MnemesisUtils};

pub fn _main(args: &ArgMatches) {
    let url         = MnemesisUtils::prompt_for_input("Url: ");
    let login       = MnemesisUtils::prompt_for_input("Login: ");
    let password    = MnemesisUtils::prompt_for_password("Password: ");
    let totp_secret = if args.is_present("totp") {
        Some(MnemesisUtils::prompt_for_input("TOTP secret: "))
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
    let utils       = MnemesisUtils::new();
    let mut list    = utils.read_entities(path);
    let concurrent  = list.iter().position(|entity| match *entity {
        Entity::Credentials(ref creds) => creds.url == credentials.url && creds.login == credentials.login,
        //_                              => false,
    });

    list.push(Entity::Credentials(credentials));

    if let Some(concurrent) = concurrent {
        list.swap_remove(concurrent);
    }

    utils.write_entities(path, list);
}
