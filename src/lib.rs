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
// Data all the way back to 2005!!

// pub mod play_by_play;
pub mod schedule;
pub mod utils;

pub (crate) const BASE_URL: &'static str = "http://statsapi.mlb.com/api/v1/";

// pub mod boxscore;

// Schedule captures all the games that are available for download. Schedule also includes a lot of metadata that we'll keep
// around and copy on write when we serialize.
// struct Schedule {

// }

// impl Schedule {
//     ///
//     /// 
//     /// 

//     fn get_year(sport_id: u8, year: u16) -> Vec<Schedule> {
//         unimplemented!()
//     }

// }
