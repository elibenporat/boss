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
//! 


mod play_by_play;

fn main() {
    
    play_by_play::parse_test_data();

}
