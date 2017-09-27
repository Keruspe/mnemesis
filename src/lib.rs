extern crate file;
extern crate rpassword;
extern crate rprompt;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate xdg;

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

#[derive(Debug)]
pub struct MnemesisUtils {
    base_dirs: BaseDirectories,
    config:    MnemesisConfig,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
struct MnemesisConfig {
    secret: String,
}

impl MnemesisUtils {
    pub fn new() -> MnemesisUtils {
        let base_dirs = BaseDirectories::with_prefix("mnemesis").expect("Failed getting base directories");
        let config    = MnemesisConfig::load(&base_dirs);

        MnemesisUtils {
            base_dirs,
            config,
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
                let data = file::get_text(full_path.to_str().expect(&format!("{:?} is not valid UTF-8", full_path))).expect(&format!("Failed to read {:?}", full_path));
                serde_json::from_str::<Vec<Entity>>(&data).expect(&format!("Found garbage in {:?}", full_path))
            } else {
                panic!("{:?} exists and is not a file", full_path);
            }
        } else {
            Vec::new()
        }
    }

    pub fn write_entities(&self, path: &str, entities: Vec<Entity>) {
        let full_path = self.credentials_directory(path);

        file::put_text(full_path.to_str().expect(&format!("{:?} is not valid UTF-8", full_path)), serde_json::to_string(&entities).expect("Failed to serialize entities")).expect(&format!("Failed to write {:?}", full_path));
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
