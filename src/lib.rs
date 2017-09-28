extern crate base64;
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
use std::str::FromStr;
use std::path::PathBuf;
use xdg::BaseDirectories;

const SERIALIZATION_API:    usize = 1;
const SERIALIZATION_FIELDS: usize = 4;

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
        let base_dirs  = BaseDirectories::with_prefix("mnemesis").expect("Failed getting base directories");
        let passphrase = MnemesisUtils::prompt_for_password("Passphrase: ");
        let config     = MnemesisConfig::load(&base_dirs, &Crypt::new(&passphrase, Algorithm::ChaCha20Poly1305), &passphrase);
        let crypt      = Crypt::new(&config.secret, Algorithm::ChaCha20Poly1305);

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
                let encrypted_data = file::get_text(full_path.to_str().expect(&format!("{:?} is not valid UTF-8", full_path))).expect(&format!("Failed to read {:?}", full_path));
                let decrypted_data = self.crypt.decrypt(encrypted_data);
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
        let decrypted_data = serde_json::to_string(&entities).expect("Failed to serialize entities");
        let encrypted_data = self.crypt.encrypt(decrypted_data);

        file::put(full_path.to_str().expect(&format!("{:?} is not valid UTF-8", full_path)), &encrypted_data).expect(&format!("Failed to write {:?}", full_path));
    }
}

impl MnemesisConfig {
    pub fn load(base_dirs: &BaseDirectories, crypt: &Crypt, passphrase: &str) -> MnemesisConfig {
        base_dirs.find_config_file("mnemesis-config.json").and_then(|conf_file| {
            conf_file.to_str().and_then(|conf_file| file::get_text(conf_file).ok()).map(|encrypted_conf| crypt.decrypt(encrypted_conf))
        }).and_then(|conf| {
            serde_json::from_str::<MnemesisConfig>(&conf).ok()
        }).unwrap_or_else(|| {
            let config = MnemesisConfig {
                secret: passphrase.to_string(),
            };
            let decrypted_data = serde_json::to_string(&config).expect("Failed to serialize config");
            let encrypted_data = crypt.encrypt(decrypted_data);
            let full_path      = base_dirs.place_config_file("mnemesis-config.json").expect("Failed to create config file");

            file::put(full_path.to_str().expect(&format!("{:?} is not valid UTF-8", full_path)), &encrypted_data).expect(&format!("Failed to write {:?}", full_path));
            config
        })
    }
}

struct Crypt {
    algorithm: Algorithm,
    key:       [u8; 32],
    rand:      rand::SystemRandom,
}

#[derive(Clone, Debug, PartialEq)]
enum Algorithm {
    ChaCha20Poly1305,
}

impl Algorithm {
    fn aead_algorithm(&self) -> &'static aead::Algorithm {
        match *self {
            Algorithm::ChaCha20Poly1305 => &aead::CHACHA20_POLY1305
        }
    }
}

impl ToString for Algorithm {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl FromStr for Algorithm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ChaCha20Poly1305" => Ok(Algorithm::ChaCha20Poly1305),
            _                   => Err(format!("Unknown algorithm: {}", s)),
        }
    }
}

impl Crypt {
    fn new(passphrase: &str, algorithm: Algorithm) -> Crypt {
        let salt      = username::get_user_name().expect("Failed to query username");
        let mut key   = [0; 32];

        pbkdf2::derive(&digest::SHA512, 100, salt.as_bytes(), passphrase.as_bytes(), &mut key);

        Crypt {
            algorithm,
            key,
            rand: rand::SystemRandom::new(),
        }
    }

    fn nonce(&self) -> Vec<u8> {
        let mut nonce = vec![0; 12];
        self.rand.fill(&mut nonce).expect("Failed generating nonce");
        nonce
    }

    fn encrypt(&self, data: String) -> String {
        let algorithm                = self.algorithm.aead_algorithm();
        let sealing_key              = aead::SealingKey::new(&algorithm, &self.key).expect("Failed creating sealing key");
        let nonce                    = self.nonce();
        let additional_data: [u8; 0] = [];
        let mut data                 = data.into_bytes();

        for _ in 0..algorithm.tag_len() {
            data.push(0);
        }

        let encrypted_data = aead::seal_in_place(&sealing_key, &nonce, &additional_data, &mut data, algorithm.tag_len()).map(|len| data[..len].to_vec()).expect("Failed sealing data");

        format!("{}:{}:{}:{}", SERIALIZATION_API, self.algorithm.to_string(), base64::encode(&nonce), base64::encode(&encrypted_data))
    }

    fn decrypt(&self, data: String) -> String {
        let components: Vec<_> = data.splitn(SERIALIZATION_FIELDS, ':').collect();

        assert_eq!(components.len(),      SERIALIZATION_FIELDS);
        assert_eq!(components[0].parse(), Ok(SERIALIZATION_API));
        assert_eq!(components[1].parse(), Ok(self.algorithm.clone())); // TODO: move me to param and use the read one

        let algorithm                = self.algorithm.aead_algorithm();
        let opening_key              = aead::OpeningKey::new(&algorithm, &self.key).expect("Failed creating opening key");
        let nonce                    = base64::decode(components[2]).expect("Failed to decode nonce");
        let mut data                 = base64::decode(components[3]).expect("Failed to decode data");
        let additional_data: [u8; 0] = [];
        let decrypted_data           = aead::open_in_place(&opening_key, &nonce, &additional_data, 0, &mut data).expect("Failed opening data");

        String::from_utf8(decrypted_data.to_vec()).expect("Failed decoding data")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let pass  = "This is a nice passphrase.";
        let c     = Crypt::new(pass, Algorithm::ChaCha20Poly1305);
        let data  = "Very secret data";
        let e     = c.encrypt(data.to_string());
        let e2    = e.clone();

        assert_eq!(c.decrypt(e), data);
        assert_eq!(Crypt::new(pass, Algorithm::ChaCha20Poly1305).decrypt(e2), data);
    }
}
