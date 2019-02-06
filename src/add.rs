use mnemesis_utils::{Credentials, Entity, MnemesisUtils};

use crate::action::AddAction;

pub fn _main(action: AddAction) {
    let url         = MnemesisUtils::prompt_for_input("Url: ");
    let login       = MnemesisUtils::prompt_for_input("Login: ");
    let password    = MnemesisUtils::prompt_for_password("Password: ");
    let totp_secret = if action.totp {
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
    let path        = action.path;
    let utils       = MnemesisUtils::new();
    let mut list    = utils.read_entities(&path);
    let concurrent  = list.iter().position(|entity| match *entity {
        Entity::Credentials(ref creds) => creds.url == credentials.url && creds.login == credentials.login,
        //_                              => false,
    });

    list.push(Entity::Credentials(credentials));

    if let Some(concurrent) = concurrent {
        list.swap_remove(concurrent);
    }

    utils.write_entities(&path, list);
}
