//! API Bindings for the MLB Schedule API hosted at http://statsapi.mlb.com/api/v1/schedule
//! All data are subject to MLB Advanced Media copyright
//! 
//! The schedule module takes two inputs, a <code>sport_id</code> that indicates the level of play, as well as a data range
//! in mm/dd//yyyy format (start and end). SportIDs are one of [1, 11, 12, 13, 14, 15, 16, 17]
//!  
//! The schedule is the starting point for gathering play by play data since it gives us a comprehensive list of all the games available to us.
//! A number of convenience functions are provided by this module to allow for easy extraction of the set of games you are looking for.
//!
//! 
//!  
//! 

use serde::{Deserialize};
use std::ops::{Range, RangeInclusive};
use std::fmt::{Display};
use crate::utils::*;

#[derive(Deserialize, Debug)]
#[serde(from = "GameDe")]
pub struct Game {
    pub game_type: GameType,
    pub game_type_desc: GameTypeDescription,
    pub game_pk: u32,
    pub game_date_year: u16,
    pub game_date_month: u8,
    pub game_date_day: u8,
    pub game_venue_id: u32,
    pub game_url_play_by_play: String,
    pub game_url_boxscore: String,
}

impl From <GameDe> for Game {
    fn from (game: GameDe) -> Game {

        let play_by_play_url = format!("{}game/{}/playByPlay", crate::BASE_URL, game.game_pk);
        let boxscore_url =     format!("{}game/{}/boxscore",   crate::BASE_URL, game.game_pk);

        Game {
            game_type: game.game_type,
            game_type_desc: game.game_type.into(),
            game_pk: game.game_pk,
            game_date_year: game.game_date.year,
            game_date_month: game.game_date.month,
            game_date_day: game.game_date.day,
            game_venue_id: game.venue.0,
            game_url_play_by_play: play_by_play_url,
            game_url_boxscore: boxscore_url,
        }
    }
} 


#[derive(Deserialize, Debug)]
#[serde(from = "Schedule")]
pub struct Schedule {
    games: Vec<Game>,
}

/// Several helper functions are provided to make pulling schedules more convenient. By default,
/// only 1 year's worth of 1 level can be queried at time, so we'll need to compose
impl Schedule {
    

    /// Get the entire schedule at the mlb level
    pub fn get_mlb_years <Y: YearRange> (years: Y) -> Schedule 
    where Y:IntoIterator, 
    <Y as IntoIterator>::Item: Display,
    {
        //MLB level sport ID
        let sport_id = 1;
        Self::get_years(sport_id, years)
    }

    fn get_years <Y: YearRange> (sport_id: u8, years: Y) -> Schedule 
    where Y:IntoIterator,
    <Y as IntoIterator>::Item: Display,
    {       
        let base_url = format!("{}/schedule?sportId={}&startDate=01/01/", crate::BASE_URL, sport_id);

        let schedule_urls: Vec<String> = years
            .into_iter()
            .map(|year| format!("{}{}&endDate=12/31/{}", base_url, year, year))
            .collect()
            ;

        let schedule_json = stream(schedule_urls);

        let games: Games = schedule_json
            .into_iter()
                .map(|json| {
                    //TODO: Add proper error handling
                    let sched: ScheduleDe = serde_json::from_str(&json.unwrap()).unwrap();
                    let games: Games = sched.into();
                    games
                })
                .flatten()
                .collect()
                ;
        Schedule {games}
    }
}

/// Use either an exclusive range such as 2012 .. 2015 or an inclusive range such as 2012 ..= 2014. YearRange is 
/// Trait-bound to the Range and RangeInclusive structs.
pub trait YearRange {}

impl YearRange for Range <usize> {}
impl YearRange for RangeInclusive <usize> {}

pub fn test_schedule() {
    // let schedule: ScheduleDe = serde_json::from_str(SCHEDULE_JSON).unwrap();
    // let games: Games = schedule.into();
    // dbg!(&games[0..10]);

    let sched = Schedule::get_mlb_years(1900 ..= 2020);
    // dbg!(&sched.games[2000]);
    dbg!(sched.games.len());
}

#[derive(Deserialize, Debug)]
struct ScheduleDe {
    dates: Vec<Dates>,
}

#[derive(Deserialize, Debug)]
struct Dates {
    // date: String,
    games: Vec<Game>,
}

pub type Games = Vec<Game>;

impl From <ScheduleDe> for Games {
    fn from (sched: ScheduleDe) -> Games {
         sched.dates.into_iter()
            .map(|date| date.games)
            .flatten()
            .collect()
    }
}

#[derive(Deserialize, Debug)]
#[serde(from="String")]
/// We'll build our own date handling rather than bring in an extra depedency. The date maths we need to do are
/// fairly simple (calculating Batter/Pitcher age) and should help keep our dependency tree as small as possible.
/// To calculate age, we'll simply do year-year + (month-month)/12 + (day-day)/30. We'll be off by a little on the day side,
/// but it doesn't really matter if we track a batter as 24.21 years old instead of 24.25 years old.
/// 
/// We ignore the time from the gameDate string as we'll get more accurate info from the boxscore, where we want the "Frist Pitch"
pub (crate) struct GameDate {
    year: u16,
    month: u8,
    day: u8,
}

impl From <String> for GameDate {
    fn from (date_string: String) -> GameDate {
        
        let date: Vec<&str> = date_string.split("T").nth(0).unwrap().split("-").collect();
        let year = date[0].parse::<u16>().unwrap();
        let month = date[1].parse::<u8>().unwrap();
        let day = date[2].parse::<u8>().unwrap();

        GameDate {
            year,
            month,
            day,
        }
    }
}




#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub (crate) struct GameDe {
    game_type: GameType,
    game_pk: u32,
    game_date: GameDate,
    venue: VenueID,
}

#[derive(Deserialize, Debug)]
#[serde(from="Venue")]
struct VenueID (u32);

impl From<Venue> for VenueID {
    fn from(venue: Venue) -> VenueID {
        VenueID(venue.id)
    }
}


#[derive(Deserialize, Debug)]
pub (crate) struct Venue { 
    id: u32,
}

#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(field_identifier)]
pub enum GameType {
    /// Regular Season
    R,
    /// First Round
    F,
    /// Division Series
    D,
    /// League Championship Series
    L,
    /// World Series
    W,
    /// Championship
    C,
    /// Nineteenth Century Series
    N,
    /// All Star Game
    A,
    /// Spring Training
    S,
    /// Exhibition Game
    E,
    /// Intrasquad
    I,
    ///Playoffs
    P,
}

#[derive(Deserialize, Debug)]
#[serde(field_identifier, from ="GameType")]
pub enum GameTypeDescription {
    RegularSeason,
    FirstRound,
    DivisionSeries,
    LeagueChampionshipSeries,
    WorldSeries,
    Championship,
    NineteenthCenturySeries,
    AllStarGame,
    SpringTraining,
    ExhibitionGame,
    Intrasqaud,
    Playoffs,
}

impl From <GameType> for GameTypeDescription {
    fn from (game_type: GameType) -> GameTypeDescription {
        match game_type {
            GameType::R => GameTypeDescription::RegularSeason,
            GameType::F => GameTypeDescription::FirstRound,
            GameType::D => GameTypeDescription::DivisionSeries,
            GameType::L => GameTypeDescription::LeagueChampionshipSeries,
            GameType::W => GameTypeDescription::WorldSeries,
            GameType::C => GameTypeDescription::Championship,
            GameType::N => GameTypeDescription::NineteenthCenturySeries,
            GameType::A => GameTypeDescription::AllStarGame,
            GameType::S => GameTypeDescription::SpringTraining,
            GameType::E => GameTypeDescription::ExhibitionGame,
            GameType::I => GameTypeDescription::Intrasqaud,
            GameType::P => GameTypeDescription::Playoffs,
        }
    }
}