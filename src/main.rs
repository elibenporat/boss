#![allow(dead_code)]
#![allow(unused)]

use statcast::GameDate;

mod statcast;
mod players;
mod date;
mod schedule;
mod cache;
mod sports;
mod play_by_play;
mod boxscore;
mod game;
mod venues;
mod metadata;
mod coaches;
mod run_expectancy;
mod team;
mod get_data;
mod nathan;

pub (crate) const BASE_URL: &'static str = "https://statsapi.mlb.com/api/v1/";
pub (crate) const BASE_URL_V11: &'static str = "https://statsapi.mlb.com/api/v1.1/";

/// Base "x" value for pixel coordinates tracked in the hitData field. This is the default value for all fields, as per the SVG files
/// If we don't have an svg file for a particular venue_id - we'll fill in the x value with this constant
pub const STADIUM_X: f32 = 125.2;
/// Base "y" value for pixel coordinates tracked in the hitData field. This is the default value for all fields, as per the SVG files
/// If we don't have an svg file for a particular venue_id - we'll fill in the y value with this constant
pub const STADIUM_Y: f32 = 203.5;

///Default number for converting pixels into feet. We'll update this variable after we analyze the data some more.
#[allow(unused)]
pub const FEET_PER_PIXEL: f32 = 2.75;

fn main() {
    
    // use reqwest::blocking::*;

    // let data = get(r#"http://statsapi.mlb.com/api/v1/game/714157/playByPlay"#).unwrap().text().unwrap_or("".to_string());
    // let pbp: crate::play_by_play::Game = serde_json::from_str(&data).expect(&format!("Error with game_pk: {}", "714157"));
    // dbg!(&data.contains("allPlays"));
    
    // let data = statcast::StatCast::download_csv("2021-04-31".into());

    // let player = players::Player::get_player(544931);
    // dbg!(player);

    get_data::get_everything();

    // let result: Vec<crate::game::Pitch> = pbp_urls.into_par_iter()
    // // .inspect(|data| println!("{}", &data.1))
    // .map (|data| (data.0, get(data.1).unwrap().text().unwrap_or("".to_string())))
    // .filter(|data| data.1.contains("allPlays"))
    // .map (|data| {
    //     let pbp: Game = serde_json::from_str(&data.1).expect(&format!("Error with game_pk: {}", data.0));
    //     let game_data = GameData {
    //         pitch_data: pbp.all_plays,
    //         meta_data: &meta_data,
    //         game_pk: data.0,
    //     };
    //     let pitches: Vec<Pitch> = game_data.into();
    //     pitches
    // })
    // .flatten()
    // .collect()
    // ;
   

    // for month in 1 ..= 12 {
    //     for day in 1 ..= 31 {
    //         let date_string = format!("2022-{:02}-{:02}",month, day);
    //         statcast::StatCast::download_csv(date_string);
    //     }
    // }
}
