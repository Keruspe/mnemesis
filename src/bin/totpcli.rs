extern crate base32;
extern crate oath;

use oath::{totp_raw_now, HashType};

use std::env;

fn main() {
    println!("{:?}", totp_raw_now(base32::decode(base32::Alphabet::RFC4648{ padding: false }, &env::args().nth(1).unwrap()).unwrap().as_ref(), 6, 0, 30, &HashType::SHA1));
}
