//! Player changes captures all player changes logged, going through the data day by data. Currently only works off of 
//! http://lookup-service-prod.mlb.com/json/named.ops_player_changes.bam?change_date=20051214, but will hopefully figure out how to get these data from stats.mlb.com/api
//! 
//! After I built it and pulled the data, looks like this is actually useless. Keeping it in here for now, but will likely remove it.
//! 
//! 
//!
//!  


use crate::utils::*;
use crate::cache;
use serde::{Deserialize, Serialize};

pub fn test_changes() {


    let mut player_changes = cache::load_player_changes();


    // Find the latest date cached
    let dates_cached:u32 = player_changes
                .iter()
                .map(|change| change.change_date)
                .max().unwrap_or(20050100)
                ;

    dbg!(dates_cached);

    let mut dates: Vec<u32> = vec![];
    for year in 2005 ..= 2019 {
        for month in 1 ..= 12 {
            for day in 1 ..= 31 {
                let date: u32 = year * 10_000 + month * 100 + day;

                //only pull dates we haven't cached
                if date > dates_cached {
                    dates.push(date);
                }
            }
        }
    }

    let links:Vec<String> = dates
                .iter()
                .map(|date| format!("http://lookup-service-prod.mlb.com/json/named.ops_player_changes.bam?change_date={}", date))
                .collect()
                ;


    let data = stream(links);
    // println!("Got the Data!");
    let changes_result: Vec<PlayerChange> = data
        .into_iter()
        .map(|json| {
            // TODO - fix the error "stream did not contain valid utf-8", right now just treating it as an empty string
            let json = json.unwrap_or("".to_string());
            serde_json::from_str(json.as_str()).unwrap_or(
                serde_json::from_str(json
                    // This is a horrible hack to placate SerDe when it expects a Vec but there's only one item. Easier to just hack the JSON than find a better workaround.
                    .replace(r#"row":"#, r#"row":["#)
                    .replace("}}}}","}]}}}")
                    .as_str())
                .unwrap_or(
                    PlayerChanges {
                        changes: vec![
                            // PlayerData {
                            //     height_feet: "".to_string(),
                            //     height_inches: "".to_string(),
                            //     weight: "".to_string(),
                            //     player_id: "".to_string(),
                            //     change_date: "".to_string(),
                            //     change: ChangeType,
                            // }
                        ],
                    }
                ))
            } 
        )
        // .inspect(|item| println!("{:?}",item))
        .map(|change| change.changes)
        .flatten()
        .map(|change| {let result: PlayerChange = change.into(); result})
        // .filter(|player| player.change_type == ChangeType::Modified)
        .collect()
        ; 
        
    // dbg!(&changes_result[0..15]);
    
    player_changes.extend(changes_result);
    
    cache::serialize_player_changes(player_changes);

}    
    
#[serde(from = "ChangeData")]
#[derive(Deserialize, Debug)]
struct PlayerChanges {
    // #[serde(default="default_change_data")]
    changes: Vec<PlayerData>,
}

// #[serde(from = "PlayerData")]
#[derive(Deserialize, Debug, Serialize)]
pub struct PlayerChange {
    pub player_id: u32,
    pub weight: Option<u16>,
    pub height_feet: Option<u8>,
    pub height_inches: Option<u8>,
    pub change_year: u32,
    pub change_month: u32,
    pub change_day: u32,
    pub change_date: u32,
    pub change_type: ChangeType,
}

impl From<PlayerData> for PlayerChange {
    fn from (player: PlayerData) -> PlayerChange {
        let date_split: Vec<&str> = player.change_date.split("T").nth(0).unwrap().split("-").collect();
        let change_year = date_split[0].parse().unwrap();
        let change_month = date_split[1].parse().unwrap();
        let change_day = date_split[2].parse().unwrap();

        PlayerChange {
            height_feet: player.height_feet.parse().ok(),
            height_inches: player.height_inches.parse().ok(),
            weight: player.weight.parse().ok(),
            player_id: player.player_id.parse().unwrap(),
            change_year,
            change_month,
            change_day,
            change_date: change_year * 10_000 + change_month * 100 + change_day,
            change_type: player.change,
        }

    }
}

impl From <ChangeData> for PlayerChanges {
    fn from (change_data: ChangeData) -> PlayerChanges {
        PlayerChanges {
            changes: change_data.ops_player_changes.query_results.changes,
        }
    }
}


#[derive(Deserialize, Debug)]
struct ChangeData {
    ops_player_changes: Changes,
}

#[serde(rename_all="camelCase")]
#[derive(Deserialize, Debug)]
struct Changes {
    query_results: QueryResults,
}


#[serde(rename_all="camelCase")]
#[derive(Deserialize, Debug)]
struct QueryResults {
    #[serde(rename="row", default="default_query_results")]
    changes: Vec<PlayerData>,
}

fn default_query_results() -> Vec<PlayerData> {
    vec![]
}

// fn default_change_data() -> Vec<Player> {
//     vec![]
// }

#[derive(Deserialize, Debug)]
struct PlayerData {
    height_feet: String,
    height_inches: String,
    weight: String,
    player_id: String,
    change_date: String,
    change: ChangeType,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all="camelCase")]
pub enum ChangeType {
    Added,
    Modified
}



