//! Error handling module 
//! 
//! TODO - Do some research on this. Need some good examples
//! 
//! 

use std::fmt::{Display, Formatter, Result};
use std::error::Error;

#[derive(Debug)]
pub enum BossError {
    Network (isahc::Error),
    JSONParse (serde_json::Error),
    IO (std::io::Error),

}

impl Display for BossError {
    fn fmt (&self, f: &mut Formatter) -> Result {
        match *self {
            BossError::Network (ref err) => write!(f, "Network Error: {}", err),
            BossError::IO (ref err) => write!(f, "Network Error: {}", err),
            BossError::JSONParse (ref err) => write!(f, "JSON Parse Error: {}", err),
        }
    }
}

impl Error for BossError {
    fn description(&self) -> &str {
        match *self {
            BossError::Network(ref err) => err.description(),
            BossError::IO(ref err) => err.description(),
            BossError::JSONParse(ref err) => err.description(),
        }
    }
    fn source (&self) -> Option<&(dyn Error +'static)> {
        match *self {
            BossError::Network(ref err) => Some(err),
            BossError::IO(ref err) => Some(err),
            BossError::JSONParse(ref err) => Some(err),
        }
    }
}

impl From<isahc::Error> for BossError {
    fn from (err: isahc::Error ) -> BossError {
        BossError::Network(err)
    }
}

impl From<serde_json::Error> for BossError {
    fn from (err: serde_json::Error ) -> BossError {
        BossError::JSONParse(err)
    }
}

impl From<std::io::Error> for BossError {
    fn from (err: std::io::Error ) -> BossError {
        BossError::IO(err)
    }
}