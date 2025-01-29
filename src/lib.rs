use std::io;
use std::str::FromStr;

pub enum OutputMode {
    Plain,
    Json,
}

impl FromStr for OutputMode {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "plain" => Ok(Self::Plain),
            "json" => Ok(Self::Json),
            _ => Err(io::Error::other(format!("unknown format: {s}"))),
        }
    }
}

pub const OUT_MODE_DEFAULT: OutputMode = OutputMode::Plain;

pub mod count;
