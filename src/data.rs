use std::collections::HashMap;

use rustc_serialize::{Encodable, Encoder, json};
use rustc_serialize::json::{DecoderError, EncoderError};

#[derive(Debug,PartialEq,RustcEncodable,RustcDecodable)]
pub struct Credentials {
    pub url:        String,
    pub login:      String,
    pub password:   String,
    pub totp_token: Option<String>
}

#[derive(Debug,PartialEq)]
pub enum Entity {
    Credentials(Credentials),
    Custom(HashMap<String, String>)
}

impl Encodable for Entity {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        match *self {
            Entity::Credentials(ref creds) => creds.encode(s),
            Entity::Custom(ref custom)     => custom.encode(s),
        }
    }
}

impl Entity {
    pub fn new_credentials(url: String, login: String, password: String, totp_token: Option<String>) -> Entity {
        Entity::Credentials(Credentials {
            url:        url,
            login:      login,
            password:   password,
            totp_token: totp_token,
        })
    }

    pub fn new_custom(data: HashMap<String, String>) -> Entity {
        Entity::Custom(data)
    }

    pub fn json_encode(&self) -> Result<String, EncoderError> {
        json::encode(self)
    }

    pub fn json_decode(data: &str) -> Result<Entity, DecoderError> {
        if let Ok(creds) = json::decode::<Credentials>(data) {
            Ok(Entity::Credentials(creds))
        } else {
            json::decode::<HashMap<String, String>>(data).map(|custom| Entity::Custom(custom))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    use rustc_serialize::json;

    fn creds1() -> Entity { Entity::new_credentials(String::from("example.com"), String::from("user"), String::from("secret"), None) }
    fn creds2() -> Entity { Entity::new_credentials(String::from("example.com"), String::from("user"), String::from("secret"), Some(String::from("123456"))) }
    const CREDS1_SERIALIZED: &'static str = "{\"url\":\"example.com\",\"login\":\"user\",\"password\":\"secret\",\"totp_token\":null}";
    const CREDS2_SERIALIZED: &'static str = "{\"url\":\"example.com\",\"login\":\"user\",\"password\":\"secret\",\"totp_token\":\"123456\"}";

    #[test]
    fn credentials_serialization() {
        let _creds1: String = creds1().json_encode().unwrap();
        let _creds2: String = creds2().json_encode().unwrap();
        assert_eq!(_creds1, CREDS1_SERIALIZED);
        assert_eq!(_creds2, CREDS2_SERIALIZED);
    }

    #[test]
    fn credentials_deserialization() {
        let _creds1: Entity = Entity::json_decode(CREDS1_SERIALIZED).unwrap();
        let _creds2: Entity = Entity::json_decode(CREDS2_SERIALIZED).unwrap();
        assert_eq!(_creds1, creds1());
        assert_eq!(_creds2, creds2());
    }

    #[test]
    fn custom_serialization() {
        let mut map = HashMap::new();
        map.insert("foo".to_string(), "bar".to_string());
        map.insert("john".to_string(), "doe".to_string());
        let custom = Entity::Custom(map);
        let serialized = json::encode(&custom).unwrap();
        assert!(serialized == "{\"foo\":\"bar\",\"john\":\"doe\"}" ||
                serialized == "{\"john\":\"doe\",\"foo\":\"bar\"}");
    }

    #[test]
    fn custom_deserialization() {
        let mut map = HashMap::new();
        map.insert("foo".to_string(), "bar".to_string());
        map.insert("john".to_string(), "doe".to_string());
        let custom  = Entity::new_custom(map);
        let decoded = Entity::json_decode("{\"foo\":\"bar\",\"john\":\"doe\"}").unwrap();
        assert_eq!(decoded, custom);
    }
}
