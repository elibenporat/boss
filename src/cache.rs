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
use crate::player_changes;
use crate::venues;
use serde::{Serialize};
use serde::de::DeserializeOwned;


const PLAYER_CHANGES_JSON: &str = "\\player_changes.json";
const VENUE_X_Y_JSON: &str = "\\venue_xy.json";

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
where T: DeserializeOwned
{
    let folder = cache_folder();
    let file_name = format!("{}{}", &folder, file_name);
    create_folder(folder);
    
    let json = std::fs::read_to_string(file_name).unwrap_or("".to_string());
    
    serde_json::from_str(&json).unwrap_or(vec![])

}

/// Serialize the venue (x,y) coordinates 
pub (crate) fn cache_venue_x_y (venues: Vec<venues::VenueXY>) {

    cache (VENUE_X_Y_JSON, venues);

}

/// Load the venue (x,y) coords from cache
pub (crate) fn load_venue_x_y () -> Vec<venues::VenueXY> {

    load (VENUE_X_Y_JSON)

}


/// Likely deprecated because the data are useless. Maybe keep it in case that changes?
pub (crate) fn serialize_player_changes (changes: Vec<player_changes::PlayerChange>) {

    cache (PLAYER_CHANGES_JSON, changes);

}

pub (crate) fn load_player_changes () -> Vec<player_changes::PlayerChange> {

    load (PLAYER_CHANGES_JSON)

}

/// Creates the cache folder if it doesn't exist.
fn create_folder (path: String) {
    match std::fs::create_dir(path) {
        Ok(dir) => {dir},
        _ => {},
    }
}