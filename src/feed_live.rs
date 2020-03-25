//! Parse the feed/live data to get the official scorer, data caster as well as construct the url for the old gameday xml files. The Stats api
//! is missing data for some old games with respect to Umpires. Additionally, the XML files contain data on player weights that are historically accurate.
//! The weight shown by the stats api is their current weight, which will generally get greater as players age. We would like to capture these data wherever possible
//! and cache the results.
//! 
//! 

use serde::{Deserialize, Serialize};


/// Contains the link for the GameDay xml folder as well as the official scorers.
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct FeedData {
    pub game_pk: u32,
    pub gameday_xml_folder: String,
    pub official_scorer_id: Option<u32>,
    pub official_scorer_name: Option<String>,
    pub primary_datacaster_id: Option<u32>,
    pub primary_datacaster_name: Option<String>,
}

impl From<Feed> for FeedData {
    fn from(feed: Feed) -> FeedData {
        let base_url = "https://gd2.mlb.com/components/game/";
        
        let mut sport_code = feed.game_data.game.id
            .clone()
            .split("-")
            .nth(0).unwrap()
            .to_string()
            ;
        let sport_code: String = sport_code.drain(sport_code.len()-3 .. ).collect();
        let game_id = feed.game_data.game.id.clone().replace("/", "_").replace("-", "_");
        let year_month_day: Vec<&str> = feed.game_data.game.id.split("/").collect();
        let (year, month, day) = (year_month_day[0], year_month_day[1], year_month_day[2]);

        let folder = format! ("{}{}/year_{}/month_{}/day_{}/gid_{}/", base_url, sport_code, year, month, day, game_id);

        let (official_scorer_id, official_scorer_name) = match feed.game_data.official_scorer {
          Some (id_name) => (id_name.id, id_name.full_name),
          _ => (None, None),
        };
        let (primary_datacaster_id, primary_datacaster_name) = match feed.game_data.primary_datacaster {
          Some (id_name) => (id_name.id, id_name.full_name),
          _ => (None, None),
        };

        FeedData {
            game_pk: feed.game_data.game.pk,
            gameday_xml_folder: folder,
            official_scorer_id,
            official_scorer_name,
            primary_datacaster_id,
            primary_datacaster_name,
        }
    }
}


#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub (crate) struct Feed {
    game_data: GameData,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub (crate) struct GameData {
    game: Game,
    official_scorer: Option<IDName>,
    primary_datacaster: Option<IDName>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
struct IDName {
    id: Option<u32>,
    full_name: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Game {
    id: String,
    pk: u32,
}


