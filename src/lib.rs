extern crate file;
extern crate rpassword;
extern crate rprompt;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate xdg;

use std::path::PathBuf;
use xdg::BaseDirectories;

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct Credentials {
    pub url:         String,
    pub login:       String,
    pub password:    String,
    pub totp_secret: Option<String>
}

pub fn prompt_for_input(msg: &str) -> String {
    rprompt::prompt_reply_stdout(msg).expect("Failed reading from stdin")
}

pub fn prompt_for_password(msg: &str) -> String {
    rpassword::prompt_password_stdout(msg).expect("Failed reading password")
}

pub fn credentials_directory(path: &str) -> PathBuf {
    BaseDirectories::with_prefix("mnemesis").expect("Failed getting base directories").place_data_file(path).expect("Failed computing credentials directory")
}

pub fn read_credentials(path: &str) -> Vec<Credentials> {
    let full_path = credentials_directory(path);

    if full_path.exists() {
        if full_path.is_file() {
            let data = file::get_text(full_path.to_str().expect(&format!("{:?} is not valid UTF-8", full_path))).expect(&format!("Failed to read {:?}", full_path));
            serde_json::from_str::<Vec<Credentials>>(&data).expect(&format!("Found garbage in {:?}", full_path))
        } else {
            panic!("{:?} exists and is not a file", full_path);
        }
    } else {
        Vec::new()
    }
}

pub fn write_credentials(path: &str, credentials: Vec<&Credentials>) {
    let full_path = credentials_directory(path);

    file::put_text(full_path.to_str().expect(&format!("{:?} is not valid UTF-8", full_path)), serde_json::to_string(&credentials).expect("Failed to serialize credentials")).expect(&format!("Failed to write {:?}", full_path));
}
