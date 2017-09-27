extern crate file;
extern crate ring;
extern crate rpassword;
extern crate rprompt;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate username;
extern crate xdg;

use ring::{aead, digest, pbkdf2, rand};
use ring::rand::SecureRandom;
use std::path::PathBuf;
use xdg::BaseDirectories;

#[derive(Debug,PartialEq,Deserialize,Serialize)]
#[serde(tag = "type")]
pub enum Entity {
    Credentials(Credentials),
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct Credentials {
    pub url:         String,
    pub login:       String,
    pub password:    String,
    pub totp_secret: Option<String>
}

pub struct MnemesisUtils {
    base_dirs: BaseDirectories,
    config:    MnemesisConfig,
    crypt:     Crypt,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
struct MnemesisConfig {
    secret: String,
}

impl MnemesisUtils {
    pub fn new() -> MnemesisUtils {
        let base_dirs = BaseDirectories::with_prefix("mnemesis").expect("Failed getting base directories");
        let config    = MnemesisConfig::load(&base_dirs);
        let crypt     = Crypt::new(&config.secret);

        MnemesisUtils {
            base_dirs,
            config,
            crypt,
        }
    }

    pub fn prompt_for_input(msg: &str) -> String {
        rprompt::prompt_reply_stdout(msg).expect("Failed reading from stdin")
    }

    pub fn prompt_for_password(msg: &str) -> String {
        rpassword::prompt_password_stdout(msg).expect("Failed reading password")
    }

    fn credentials_directory(&self, path: &str) -> PathBuf {
        self.base_dirs.place_data_file(path).expect("Failed computing credentials directory")
    }

    pub fn read_entities(&self, path: &str) -> Vec<Entity> {
        let full_path = self.credentials_directory(path);

        if full_path.exists() {
            if full_path.is_file() {
                let encrypted_data = file::get(full_path.to_str().expect(&format!("{:?} is not valid UTF-8", full_path))).expect(&format!("Failed to read {:?}", full_path));
                let nonce          = file::get(full_path.with_extension("nonce").to_str().expect(&format!("{:?} is not valid UTF-8", full_path))).expect(&format!("Failed to read {:?}", full_path));
                let decrypted_data = self.crypt.decrypt(encrypted_data, &nonce);
                serde_json::from_str::<Vec<Entity>>(&decrypted_data).expect(&format!("Found garbage in {:?}", full_path))
            } else {
                panic!("{:?} exists and is not a file", full_path);
            }
        } else {
            Vec::new()
        }
    }

    pub fn write_entities(&self, path: &str, entities: Vec<Entity>) {
        let full_path      = self.credentials_directory(path);
        let nonce          = self.crypt.nonce();
        let decrypted_data = serde_json::to_vec(&entities).expect("Failed to serialize entities");
        let encrypted_data = self.crypt.encrypt(decrypted_data, &nonce);

        file::put(full_path.to_str().expect(&format!("{:?} is not valid UTF-8", full_path)), &encrypted_data).expect(&format!("Failed to write {:?}", full_path));
        file::put(full_path.with_extension("nonce").to_str().expect(&format!("{:?} is not valid UTF-8", full_path)), &nonce).expect(&format!("Failed to write {:?}", full_path));
    }
}

impl MnemesisConfig {
    pub fn load(base_dirs: &BaseDirectories) -> MnemesisConfig {
        base_dirs.find_config_file("mnemesis-config.json").and_then(|conf_file| {
            conf_file.to_str().and_then(|conf_file| file::get_text(conf_file).ok())
        }).and_then(|conf| {
            serde_json::from_str::<MnemesisConfig>(&conf).ok()
        }).unwrap_or(MnemesisConfig {
            // FIXME: generate default config or error out?
            secret: "".to_string(),
        })
    }
}

struct Crypt {
    key:  [u8; 32],
    rand: rand::SystemRandom,
}

impl Crypt {
    fn new(passphrase: &str) -> Crypt {
        let salt      = username::get_user_name().expect("Failed to query username");
        let mut key   = [0; 32];

        pbkdf2::derive(&digest::SHA512, 100, salt.as_bytes(), passphrase.as_bytes(), &mut key);

        Crypt {
            key,
            rand: rand::SystemRandom::new(),
        }
    }

    fn nonce(&self) -> Vec<u8> {
        let mut nonce = vec![0; 12];
        self.rand.fill(&mut nonce).expect("Failed generating nonce");
        nonce
    }

    fn encrypt(&self, mut data: Vec<u8>, nonce: &Vec<u8>) -> Vec<u8> {
        let sealing_key              = aead::SealingKey::new(&aead::CHACHA20_POLY1305, &self.key).expect("Failed creating sealing key");
        let additional_data: [u8; 0] = [];

        for _ in 0..aead::CHACHA20_POLY1305.tag_len() {
            data.push(0);
        }

        aead::seal_in_place(&sealing_key, &nonce, &additional_data, &mut data, aead::CHACHA20_POLY1305.tag_len()).map(|len| data[..len].to_vec()).expect("Failed sealing data")
    }

    fn decrypt(&self, mut data: Vec<u8>, nonce: &Vec<u8>) -> String {
        let opening_key              = aead::OpeningKey::new(&aead::CHACHA20_POLY1305, &self.key).expect("Failed creating opening key");
        let additional_data: [u8; 0] = [];
        let decrypted_data           = aead::open_in_place(&opening_key, nonce, &additional_data, 0, &mut data).expect("Failed opening data");

        String::from_utf8(decrypted_data.to_vec()).expect("Failed decoding data")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let pass  = "This is a nice passphrase.";
        let c     = Crypt::new(pass);
        let nonce = c.nonce();
        let data  = "Very secret data";
        let e     = c.encrypt(data.as_bytes().to_vec(), &nonce);
        let e2    = e.clone();

        assert_eq!(c.decrypt(e, &nonce), data);
        assert_eq!(Crypt::new(pass).decrypt(e2, &nonce), data);
    }
}
