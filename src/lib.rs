extern crate rpassword;
extern crate rprompt;

pub fn prompt_for_input(msg: &str) -> String {
    rprompt::prompt_reply_stdout(msg).expect("Failed reading from stdin")
}

pub fn prompt_for_password(msg: &str) -> String {
    rpassword::prompt_password_stdout(msg).expect("Failed reading password")
}
