use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Raw,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Raw
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Mode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "raw" => Ok(Mode::Raw),
            _     => Err(()),
        }
    }
}
