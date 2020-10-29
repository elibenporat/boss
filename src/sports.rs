//! Module for tracking level of play, referred to as "Sport" by the API. The list of sports are rather limited, so we'll hardcode
//! everything manually here. This will allow us the flexibility to group various levels together for a better end-user experience.
//! 
//! The full list of sports can be found at http://statsapi.mlb.com/api/v1/sports/ and as of Dec 2019, they are all hardcoded in this module.
//! 
//! 

use serde::{Serialize, Deserialize};
use tree_buf::{Read, Write};

pub fn test_sports() {
    
    dbg! (get_sport(1));
    dbg! (get_all_sport_ids());
}


///Returns all sport ids available to be pulled. Use this function if you want ALL the data
pub fn get_all_sport_ids () -> Vec<u32> {
    SPORTS
        .iter()
        .map(|sport| sport.id)
        .collect()
}

pub (crate) fn get_sport(id: u32) -> Sport {
    *SPORTS
        .iter()
        .filter(|sport| sport.id == id)
        .nth(0)
        // We control the ids fed to this function and make sure we feed it a valid id
        .unwrap()
}

#[derive(Debug, Copy, Clone)]
pub struct Sport {
    pub id: u32,
    pub code: &'static str,
    pub name: &'static str,
    pub abbr: &'static str,
    pub affiliation: MLB,
    pub professional: Pro,
    pub international: International,
    /// Made up term by the crate author. This allows for easy sorting by level of play, with 0 = MLB. 
    /// For unaffiliated ball, subjective judgements will be made as to what the appropriate rank is.
    pub level_of_play_rank: u8,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Read, Write, PartialEq)]
pub enum MLB {
    MLB,
    Minors,
    Unaffiliated,
}

#[derive(Debug, Copy, Clone)]
pub enum Pro {
    Professional,
    Amateur,
    Either,
}

#[derive(Debug, Copy, Clone)]
pub enum International {
    NorthAmerica,
    International,
    Either,
}

pub (crate) const SPORTS: [Sport; 21] = 
    [
        Sport {
            id: 1,
            code: "mlb",
            name: "MLB",
            abbr: "MLB",
            level_of_play_rank: 0,
            affiliation: MLB::MLB,
            professional: Pro::Professional,
            international: International::NorthAmerica,
        },
        Sport {
            id: 11,
            code: "aaa",
            name: "Triple-A",
            abbr: "AAA",
            level_of_play_rank: 1,
            affiliation: MLB::Minors,
            professional: Pro::Professional,
            international: International::NorthAmerica,
        },
        Sport {
            id: 12,
            code: "aax",
            name: "Double-A",
            abbr: "AA",
            level_of_play_rank: 2,
            affiliation: MLB::Minors,
            professional: Pro::Professional,
            international: International::NorthAmerica,
        },
        Sport {
            id: 13,
            code: "afa",
            name: "Class A Advanced",
            abbr: "A+",
            level_of_play_rank: 3,
            affiliation: MLB::Minors,
            professional: Pro::Professional,
            international: International::NorthAmerica,
        },
        Sport {
            id: 14,
            code: "afx",
            name: "Class A",
            abbr: "A",
            level_of_play_rank: 4,
            affiliation: MLB::Minors,
            professional: Pro::Professional,
            international: International::NorthAmerica,
        },
        Sport {
            id: 15,
            code: "asa",
            name: "Class A Short Season",
            abbr: "A-",
            level_of_play_rank: 5,
            affiliation: MLB::Minors,
            professional: Pro::Professional,
            international: International::NorthAmerica,
        },
        Sport {
            id: 5442,
            code: "roa",
            name: "Rookie Advanced",
            abbr: "R+",
            level_of_play_rank: 6,
            affiliation: MLB::Minors,
            professional: Pro::Professional,
            international: International::NorthAmerica,
        },
        Sport {
            id: 16,
            code: "rok",
            name: "Rookie",
            abbr: "R",
            level_of_play_rank: 7,
            affiliation: MLB::Minors,
            professional: Pro::Professional,
            international: International::NorthAmerica,
        },
        Sport {
            id: 17,
            code: "win",
            name: "Winter Leagues",
            abbr: "WIN",
            level_of_play_rank: 8,
            affiliation: MLB::Minors,
            professional: Pro::Professional,
            international: International::Either,
        },
        Sport {
            id: 8,
            code: "bbl",
            name: "Organized Baseball",
            abbr: "PRO",
            level_of_play_rank: 9,
            affiliation: MLB::Unaffiliated,
            professional: Pro::Professional,
            international: International::Either,
        },
        Sport {
            id: 21,
            code: "min",
            name: "Minor League Baseball",
            abbr: "MIN",
            level_of_play_rank: 9,
            affiliation: MLB::Minors,
            professional: Pro::Professional,
            international: International::NorthAmerica,
        },
        Sport {
            id: 23,
            code: "ind",
            name: "Independent Leagues",
            abbr: "IND",
            level_of_play_rank: 9,
            affiliation: MLB::Unaffiliated,
            professional: Pro::Professional,
            international: International::NorthAmerica,
        },
        Sport {
            id: 31,
            code: "jml",
            name: "Japanese Major Leauge Baseball",
            abbr: "JML",
            // Treating Level of Play = AAA Baseball. 
            level_of_play_rank: 1,
            affiliation: MLB::Unaffiliated,
            professional: Pro::Professional,
            international: International::International,
        },
        Sport {
            id: 51,
            code: "int",
            name: "International Baseball",
            abbr: "INT",
            level_of_play_rank: 10,
            affiliation: MLB::Unaffiliated,
            professional: Pro::Either,
            international: International::International,
        },
        Sport {
            id: 508,
            code: "nat",
            name: "International Baseball (Collegiate)",
            abbr: "INTC",
            level_of_play_rank: 10,
            affiliation: MLB::Unaffiliated,
            professional: Pro::Amateur,
            international: International::International,
        },
        Sport {
            id: 509,
            code: "nae",
            name: "International Baseball (18 and Under)",
            abbr: "18U",
            level_of_play_rank: 10,
            affiliation: MLB::Unaffiliated,
            professional: Pro::Amateur,
            international: International::International,
        },
        Sport {
            id: 600,
            code: "nav",
            name: "International Baseball (17 and Under)",
            abbr: "17U",
            level_of_play_rank: 10,
            affiliation: MLB::Unaffiliated,
            professional: Pro::Amateur,
            international: International::International,
        },
        Sport {
            id: 510,
            code: "nas",
            name: "International Baseball (16 and Under)",
            abbr: "16U",
            level_of_play_rank: 10,
            affiliation: MLB::Unaffiliated,
            professional: Pro::Amateur,
            international: International::International,
        },
        Sport {
            id: 512,
            code: "nas",
            name: "Women's Baseball",
            abbr: "WBB",
            level_of_play_rank: 10,
            affiliation: MLB::Unaffiliated,
            professional: Pro::Either,
            international: International::International,
        },
        Sport {
            id: 22,
            code: "bbc",
            name: "College Baseball",
            abbr: "CBB",
            // Setting it at the level of Class A Short Season
            level_of_play_rank: 5,
            affiliation: MLB::Unaffiliated,
            professional: Pro::Amateur,
            international: International::NorthAmerica,
        },
        Sport {
            id: 586,
            code: "hsb",
            name: "High School Baseball",
            abbr: "HSB",
            // Setting it at the level of Rookie Ball
            level_of_play_rank: 7,
            affiliation: MLB::Unaffiliated,
            professional: Pro::Amateur,
            international: International::NorthAmerica,
        },
    ];
