#![warn(rust_2018_idioms)]

use base32;
use oath::{totp_raw_now, HashType};

use std::env;

fn main() {
    println!("{:06}", totp_raw_now(base32::decode(base32::Alphabet::RFC4648{ padding: false }, &env::args().nth(1).unwrap()).unwrap().as_ref(), 6, 0, 30, &HashType::SHA1));
}
