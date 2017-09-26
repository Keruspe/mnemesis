#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct Credentials {
    pub url:        String,
    pub login:      String,
    pub password:   String,
    pub totp_token: Option<String>
}

#[cfg(test)]
mod tests {
    use super::*;
}
