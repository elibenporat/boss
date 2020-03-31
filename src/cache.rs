//! # Caching module for BOSS
//! Provides caching for data pulled from the network. Data that has been pulled from the network, generally never needs to be pulled again.
//! For the vast majority of the meta-data, such as Venues, Teams and Players, these data don't change very much. This module will store the smaller data sets as JSON and 
//! use a custom, highly compact serialization for the pitch by pitch data.
//! 
//! This module is perhaps the most important as it will allow for easy incremental pulls. It will also serve as a demonstration for how efficicently a game can be stored.
//! The scraping modules will all need to be aware of the serialized files and have an Overwrite enum to allow for refreshing the data for whatever reason. All serialized data will
//! be stored in the "/cache" folder with locations specified as consts
//! 
//! # Groups of Data
//! 
//! ## Venues
//! 
//! Venues have 3 layers of data. First, there is the unchanging aspect of each venue, such as the location. Second, we have per-season data which will vary for dimension as well as capacity and
//! possibly the venue name as well. Third, we have the SVG picture of the venue, from which we'll extract the (X,Y) coords of home plate. We'll also want to compute dimension from the play by play data
//! so that we can show the home run fence distance at 10 different points (0 degrees, 10 degrees ..= 90 degrees).
//! 
//! ## Players
//! Players have a "current" bio which is easy to query. They also have a lot of changes to their bio, specifically for their height and weight (mostly weight). Height and Weight are both good predictors of exit velo,
//! so if we're projecting prospects, we want to know what they weighed early in their career.
//! 
//! 

use crate::utils;
use crate::venues;
use crate::schedule;
use crate::boxscore;
use crate::feed_live;
use crate::coaches;
use crate::players;
use crate::team;
use crate::game;
use serde::Serialize;
use serde::de::DeserializeOwned;
use csv::{Reader, Writer};


const VENUE_X_Y_JSON: &str = "\\venue_xy.json";
const VENUE_JSON: &str = "\\venues.json";
const SCHEDULE_JSON: &str = "\\schedule.json";
const FEED_LIVE_JSON: &str = "\\feed_live.json";
const BOXSCORE_JSON: &str = "\\boxscore.json";
const COACH_JSON: &str = "\\coaches.json";
const PLAYER_JSON: &str = "\\players.json";
const TEAMS_JSON: &str = "\\teams.json";
const PLAY_BY_PLAY: &str = r#"F:\Baseball\baseball.csv"#;

fn cache_folder () -> String {
    format!("{}{}", utils::get_directory(), "\\cache" )
}


fn cache <T> (file_name: &str, data: Vec<T>) 
where T: Serialize
{

    let folder = cache_folder();
    let file_name = format!("{}{}", &folder, file_name);
    let json = serde_json::to_string(&data).unwrap();
    create_folder(folder);
    std::fs::write(file_name, json).unwrap();

}

fn load <T> (file_name: &str) -> Vec<T>
where T: DeserializeOwned,
// T: std::fmt::Debug,
{
    let folder = cache_folder();
    let file_name = format!("{}{}", &folder, file_name);
    create_folder(folder);
    
    let json = std::fs::read_to_string(file_name).unwrap_or("".to_string());

    // let dbg_info: Result<T, serde_json::Error> = serde_json::from_str(&json);
    // if dbg_info.is_err() {
    //     println!("Error: {:?}", dbg_info);
    // };
    
    serde_json::from_str(&json).unwrap_or(vec![])
    

}

pub (crate) fn write_play_by_play (pitches: &Vec<game::Pitch>) {

    let mut csv_writer = Writer::from_path(PLAY_BY_PLAY).unwrap();

    for pitch in pitches {
        csv_writer.serialize(pitch).unwrap();
    };

}

pub (crate) fn load_play_by_play () -> Vec<game::Pitch> {

    let mut csv_reader = Reader::from_path(PLAY_BY_PLAY).unwrap();

    type CSVResult = Result< game::Pitch, csv::Error>;

    csv_reader.deserialize()
        .filter_map(|record: CSVResult| record.ok())
        // .map(|record| {let pitch: game::Pitch = record.unwrap(); pitch})
        .collect()
        
}

pub (crate) fn cache_teams_data (teams: &Vec<team::TeamData>) {
    cache (TEAMS_JSON, teams.clone());
}

pub (crate) fn load_teams_data () -> Vec<team::TeamData> {
    load (TEAMS_JSON)
}


pub(crate) fn cache_player_data (players: &Vec<players::Player>) {
    cache (PLAYER_JSON, players.clone());
}

pub (crate) fn load_player_data () -> Vec<players::Player> {
    load (PLAYER_JSON)
}

pub(crate) fn cache_coach_data (coaches: &Vec<coaches::CoachData>) {
    cache (COACH_JSON, coaches.clone());
}

pub (crate) fn load_coach_data () -> Vec<coaches::CoachData> {
    load (COACH_JSON)
}

pub(crate) fn cache_boxscore_data (boxscores: &Vec<boxscore::BoxScoreData>) {
    cache (BOXSCORE_JSON, boxscores.clone());
}

pub (crate) fn load_boxscore_data () -> Vec<boxscore::BoxScoreData> {
    load (BOXSCORE_JSON)
}


pub(crate) fn cache_feed_live_data (games: &Vec<feed_live::FeedData>) {
    cache (FEED_LIVE_JSON, games.clone());
}

pub (crate) fn load_feed_live_data () -> Vec<feed_live::FeedData> {
    load (FEED_LIVE_JSON)
}


///Serialize the schedule data
pub (crate) fn cache_schedule (games: &Vec<schedule::GameMetaData>) {  
    cache (SCHEDULE_JSON, games.clone());
}

///Load the chedule data
pub (crate) fn load_schedule () -> Vec<schedule::GameMetaData> {
    load (SCHEDULE_JSON)
} 

/// Serialize the venue (x,y) coordinates 
pub (crate) fn cache_venue_x_y (venues: &Vec<venues::VenueXY>) {

    cache (VENUE_X_Y_JSON, venues.clone());

}

/// Load the venue (x,y) coords from cache
pub (crate) fn load_venue_x_y () -> Vec<venues::VenueXY> {

    load (VENUE_X_Y_JSON)

}

/// Serialize the venue (x,y) coordinates 
pub (crate) fn cache_venue (venues: &Vec<venues::VenueData>) {

    cache (VENUE_JSON, venues.clone());

}

/// Load the venue (x,y) coords from cache
pub (crate) fn load_venue () -> Vec<venues::VenueData> {

    load (VENUE_JSON)

}


/// Creates the cache folder if it doesn't exist.
fn create_folder (path: String) {
    match std::fs::create_dir(path) {
        Ok(dir) => {dir},
        _ => {},
    }
}