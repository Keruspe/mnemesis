use mnemesis_utils::{MnemesisUtils};

use action::GetAction;
use mode::Mode;

pub fn _main(action: GetAction) {
    let path   = action.path;
    let utils  = MnemesisUtils::new();
    let entity = utils.read_entity(&path);

    if let Some(entity) = entity {
        match action.mode {
            Mode::Raw => println!("{}", entity),
        }
    }
}
