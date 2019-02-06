use clap::ArgMatches;

use std::str::FromStr;

use crate::mode::Mode;

pub enum Action {
    Add(AddAction),
    Get(GetAction),
}

pub struct AddAction {
    pub path: String,
    pub totp: bool,
}

pub struct GetAction {
    pub path: String,
    pub mode: Mode,
}

impl Action {
    pub fn from_matches(matches: ArgMatches<'_>) -> Result<Action, String> {
        if let Some(sub) = matches.subcommand_matches("add") {
            Ok(Action::Add(AddAction {
                path: sub.value_of("PATH").unwrap().to_string(),
                totp: sub.is_present("totp"),
            }))
        } else if let Some(sub) = matches.subcommand_matches("get") {
            Ok(Action::Get(GetAction {
                path: sub.value_of("PATH").unwrap().to_string(),
                mode: sub.value_of("mode").ok_or(()).and_then(Mode::from_str).unwrap_or_else(|_| Mode::default()),
            }))
        } else if matches.is_present("PATH") {
            Ok(Action::Get(GetAction {
                path: matches.value_of("PATH").unwrap().to_string(),
                mode: matches.value_of("mode").ok_or(()).and_then(Mode::from_str).unwrap_or_else(|_| Mode::default()),
            }))
        } else {
            Err(matches.usage().to_string())
        }
    }
}
