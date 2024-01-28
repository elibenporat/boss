//! API Bindings for the MLB Schedule API hosted at https://statsapi.mlb.com/api/v1/schedule
//! All data are subject to MLB Advanced Media copyright
//! 
//! The schedule module takes two inputs, a <code>sport_id</code> that indicates the level of play, as well as a date range
//! in mm/dd//yyyy format (start and end). SportIDs are one of [1, 11, 12, 13, 14, 15, 16, 17] etc.
//!  
//! The schedule is the starting point for gathering play by play data since it gives us a comprehensive list of all the games available to us.
//! To use the schedule efficiently, you need to load the "cached" schedules, ie all the schedule data that you've already downloaded. This will
//! keep track of seasons where we have some games that are not "Final". This allows us to only update seasons that are in progress, but also make sure we
//! pick up any missing seasons.
//!
//! Important "Hydrations" include:
//! * gameInfo (has attendance and first pitch)
//! * weather (instead of getting from the boxscore)
//! 
//! 

use serde::{Deserialize, Serialize};
use std::ops::{Range, RangeInclusive};

use crate::sports;
use crate::cache::{cache, load};
use rayon::prelude::*;
use reqwest::blocking::get as download;
use std::collections::{BTreeSet, BTreeMap};
use crate::date::Date;

// ///Serialize the schedule data
// pub (crate) fn cache_schedule (games: &Vec<schedule::GameMetaData>) {  
//     cache (SCHEDULE_JSON, games.clone());
// }

// ///Load the chedule data
// pub (crate) fn load_schedule () -> Vec<schedule::GameMetaData> {
//     load (SCHEDULE_JSON)
// } 
const SCHEDULE_JSON: &str = "\\schedule.json";

pub type SeasonSportStatus = BTreeSet<(u16, u32, AbstractGameState)>;
pub type SeasonSportCache = BTreeMap<(u16, u32), SeasonStatus>;


///GameMetaData is the struct exported by the schedule module.
#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct GameMetaData {
    pub game_type: GameType,
    pub game_type_desc: GameTypeDescription,
    pub game_pk: u32,
    pub game_date: GameDate,
    pub game_venue_id: u32,
    pub game_url_play_by_play: String,
    pub game_url_boxscore: String,
    pub coaches_home_url: String,
    pub coaches_away_url: String,
    pub game_status: AbstractGameState,
    pub sport_id: u32,
}

impl From<GameWithSportId> for GameMetaData {
    fn from (game: GameWithSportId) -> GameMetaData {
        GameMetaData {
            game_type: game.game.game_type,
            game_type_desc: game.game.game_type_desc,
            game_pk: game.game.game_pk,
            game_date: game.game.game_date,
            game_venue_id: game.game.game_venue_id,
            game_url_play_by_play: game.game.game_url_play_by_play,
            game_url_boxscore: game.game.game_url_boxscore,
            coaches_home_url: game.game.coaches_home_url,
            coaches_away_url: game.game.coaches_away_url,
            game_status: game.game.game_status,
            sport_id: game.sport_id,
        }
    }
}


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(from = "GameDe")]
pub struct Game {
    pub game_type: GameType,
    pub game_type_desc: GameTypeDescription,
    pub game_pk: u32,
    pub game_date: GameDate,
    pub game_venue_id: u32,
    pub game_url_play_by_play: String,
    pub game_url_boxscore: String,
    pub game_url_feed_live: String,
    pub coaches_home_url: String,
    pub coaches_away_url: String,
    pub game_status: AbstractGameState,
}


impl From <GameDe> for Game {
    fn from (game: GameDe) -> Game {

        let game_url_play_by_play = format!("{}game/{}/playByPlay",  crate::BASE_URL, game.game_pk);
        let game_url_boxscore =     format!("{}game/{}/boxscore",    crate::BASE_URL, game.game_pk);
        let game_url_feed_live =    format!("{}game/{}/feed/live",   crate::BASE_URL_V11, game.game_pk);
        let game_date: GameDate = game.game_date.into();
        let coaches_home_url =      format!("{}teams/{}/coaches/?date={}/{}/{}",crate::BASE_URL, game.teams.home.team.id, game_date.month, game_date.day, game_date.year);
        let coaches_away_url =      format!("{}teams/{}/coaches/?date={}/{}/{}",crate::BASE_URL, game.teams.away.team.id, game_date.month, game_date.day, game_date.year);

        Game {
            game_type: game.game_type,
            game_type_desc: game.game_type.into(),
            game_pk: game.game_pk,
            game_date,
            game_venue_id: game.venue.0,
            game_url_play_by_play,
            game_url_boxscore,
            game_url_feed_live,
            coaches_home_url,
            coaches_away_url,
            game_status: game.status.abstract_game_state,
        }
    }
} 


#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
pub enum SeasonStatus {
    Complete,
    Partial,
    None,
}

impl SeasonStatus {
    pub fn status (year: u16, sport_id: u32, cache: &SeasonSportStatus) -> SeasonStatus {
        let has_final_games = cache.contains(&(year, sport_id, AbstractGameState::Final));
        let has_non_final_games = cache.contains(&(year, sport_id, AbstractGameState::NotFinal));
    
        match (has_final_games, has_non_final_games) {
            ( true , false  ) => SeasonStatus::Complete,
            ( false, false  ) => SeasonStatus::None,
            ( _    , _      ) => SeasonStatus::Partial,
        }
    }
}



#[derive(Deserialize, Serialize, Debug)]
#[serde(from = "Schedule")]
pub struct Schedule {
    pub games: Games,
}

impl Schedule {   

    pub fn update () -> Self {
        todo!()
    }

    fn download_years (years: Vec<u16>, sport_id: u32, cache: &SeasonSportCache) -> Schedule {
        let base_url = format!("{}schedule?sportId={}&startDate=01/01/", crate::BASE_URL, sport_id);

        // Build the list of URLs to query, filtering out any years where we have complete seasons.
        let schedule_urls: Vec<(String, u32)> = years
            .into_iter()
            .filter(|year| cache.get(&(*year, sport_id)) != Some(&SeasonStatus::Complete))
            .map(|year| (format!("{}{}&endDate=12/31/{}", base_url, year, year), sport_id))
            .collect()
            ;

        let games: Games = schedule_urls
        .into_par_iter()
            .map(|url| {
                //TODO: Add proper error handling
                // println!("Downloading: {}", &url.0);
                let json: String = download(url.clone().0).unwrap().text().unwrap();
                let sched: ScheduleDe = serde_json::from_str(&json).unwrap();
                let sched_with_context = ScheduleWithContext {
                    sched, 
                    sport_id: url.1,
                };
                let games: Games = sched_with_context.into();
                games
            })
            .flatten()
            .collect()
            ;
        Schedule {games}
    }


    pub fn get_data (years: Vec<u16>, sport_ids: Vec<u32>, cache: &SeasonSportCache) -> Schedule 
    {
        let games: Games = sport_ids
            .into_iter()
            .map(|sport_id| Self::download_years(years.clone(), sport_id, &cache).games)
            .flatten()
            .collect()
            ;
        Schedule {games}
    }

}

struct ScheduleWithContext {
    sched: ScheduleDe,
    sport_id: u32,
}

/// Use either an exclusive range such as 2012 .. 2015 with YearRange::from_range or an 
/// inclusive range such as 2012 ..= 2014 with YearRange::from_range.
pub struct YearRange;

impl YearRange {
    pub fn from_range_inc(range: RangeInclusive<u16>) -> Vec<u16> {
        (*range.start() ..= *range.end()).into_iter().collect()
    }
    pub fn from_range(range: Range<u16>) -> Vec<u16> {
        (range.start .. range.end).into_iter().collect()
    }
}

#[derive(Deserialize, Debug)]
struct ScheduleDe {
    dates: Vec<Dates>,
}

#[derive(Deserialize, Debug, Clone)]
struct Dates {
    // date: String,
    games: Vec<Game>,
}

/// GameWithSportId is a temporary struct we need to get deserialization to work properly.
/// This gets converted into a flat GameMetaData that is exported by this module.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameWithSportId {
    game: Game,
    sport_id: u32,
}

pub type Games = Vec<GameWithSportId>;

impl From <ScheduleWithContext> for Games {
    fn from (sched: ScheduleWithContext) -> Games {
         sched.sched.dates.clone().into_iter()
            .map(|date| date.games)
            .flatten()
            .map(|game| 
                GameWithSportId {
                    game, 
                    sport_id: sched.sport_id,
                }
            )
            .collect()
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct GameDate {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl From <crate::date::Date> for GameDate {
    fn from(value: crate::date::Date) -> Self {
        Self {
            year: value.year,
            month: value.month,
            day: value.day
        }
    }
}

impl ToString for GameDate {
    fn to_string(&self) -> String {
        format!("{}-{}-{}", self.year, self.month, self.day)
    }
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
/// Get the difference between two dates in units of years. Does a rough approximation only. This is primarily used for computing
/// the age of the batter and does not need to be super precise. This saves us from having to import a library to do proper date
/// math. This may change in the future.
impl std::ops::Sub for GameDate {
    type Output = f32;

    fn sub(self, other:GameDate) -> f32 {
        
        let year_diff = self.year as f32 - other.year as f32;
        let month_diff = self.month as f32 - other.month as f32;
        let day_diff = self.day as f32 - other.day as f32;

        year_diff + (month_diff / 12f32 ) + (day_diff / 365.25f32)
    
    }

}



#[derive(Deserialize, Debug)]
pub (crate) struct GameDe {
    #[serde(alias="gameType")]
    game_type: GameType,
    #[serde(alias="gamePk")]
    game_pk: u32,
    #[serde(alias="gameDate")]
    game_date: String,
    teams: Teams,
    venue: VenueID,
    status: GameStatus,
}

#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all="camelCase")]
struct GameStatus {
    abstract_game_state: AbstractGameState,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum AbstractGameState {
    Final,
    #[serde(other)]
    NotFinal,
}

#[derive(Deserialize, Debug)]
struct Teams {
    away: Team,
    home: Team,
}

#[derive(Deserialize, Debug)]
struct Team {
    team: ID,
}

#[derive(Deserialize, Debug)]
struct ID {
    id: u32,
}



#[derive(Deserialize, Debug)]
#[serde(from="Venue")]
struct VenueID (u32);

impl From<Venue> for VenueID {
    fn from(venue: Venue) -> VenueID {
        // 4270 is the "generic" stadium
        VenueID(venue.id.unwrap_or(4270))
    }
}


#[derive(Deserialize, Debug)]
struct Venue { 
    id: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
// #[serde(field_identifier)]
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

#[derive(Deserialize, Serialize, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
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