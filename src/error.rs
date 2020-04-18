//! Error handling module for BOSS
//! 
//! There are a lot of things that can go wrong when we try to parse and build out a baseball game into data. 
//! 
//! Errors can be grouped as follows:
//! 
//! **Source Data Issues:**\
//!     * **Extra Records / Garbage Records:** Certain source data are clearly junk as they produce counts (balls and strikes) that are clearly wrong.
//!     * **Missing Metadata:** Some games do not have any boxscore data, or are missing other metadata such as coaches, venue etc.
//!     * 
//! 
//! 
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