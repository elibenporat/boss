//! BOSS | Baseball Open Source Software
//! ===
//!
//! A pure Rust baseball data aggregation and analytics library. Supports data aggregation from a number of sources
//! including the MLB stats API, MLB gameday files. Eventually, other sources such as RetroSheet and NCAA will be added.
//!  
//! BOSS is designed from the ground up to be extremely efficient. ALl text fields that can be converted to an enum have been
//! carefully mapped. The challenge with baseball data isn't the computational complexity of data gathering, it is the sheer
//! size of the data set. One of BOSS' primary design goals is to be as efficient as possible.
//! 
//! Perhaps the biggest performance optimization in BOSS is the use of parallel asynchronous streams. Most of the time this 
//! library will spend will be waiting on data from the network. Being able to stream process items as they become available is 
//! critical to having a performant application. 
//! 
//! 



// const SPORT_IDS: [u8; 8] = [1, 11, 12, 13, 14, 15, 16, 17];
// http://lookup-service-prod.mlb.com/json/named.player_id_xref.bam?player_id=545361
// Data all the way back to 2005
// Also some college baseball with exit velos??? http://statsapi.mlb.com/api/v1/sports
// 23 = Independent Leagues
// 31 = Japan (no data there that I can see)
// 51 = International ? not sure what this is
// 512 = Women's Baseball
// 22 = College Baseball / 586 = High School Baseball
// Get coaches via: http://statsapi.mlb.com/api/v1/teams/110/coaches/?date=06/19/2015 (need team ID and date)
// Look at this and add as prior art : https://github.com/toddrob99/MLB-StatsAPI/blob/master/statsapi/endpoints.py


pub mod play_by_play;
pub mod schedule;
pub mod utils;
pub mod boxscore;
pub mod player_changes;
pub mod venues;

pub (crate) const BASE_URL: &'static str = "http://statsapi.mlb.com/api/v1/";

/// CHUNK_SIZE controls how many files we request from the network at a time. This is to reduce the probability of network timeouts from flooding too many requests at once.
/// This value will be used in stream_chunked. Only use this if you get a network timeout error.
pub const CHUNK_SIZE: usize = 30;

/// Base "x" value for pixel coordinates tracked in the hitData field. This is the default value for all fields, as per the SVG files
#[allow(unused)]
pub const STADIUM_X: f32 = 123.7;
/// Base "y" value for pixel coordinates tracked in the hitData field. This is the default value for all fields, as per the SVG files
#[allow(unused)]
pub const STADIUM_Y: f32 = 201.9;

