//! Player changes captures all player changes logged, going through the data day by data. Currently only works off of 
//! http://lookup-service-prod.mlb.com/json/named.ops_player_changes.bam?change_date=20051214, but will hopefully figure out how to get these data from stats.mlb.com/api
//! 
//! 
//!
//!  


use crate::utils::*;
use serde::Deserialize;

pub fn test_changes() {

    let mut dates: Vec<u32> = vec![];
    for year in 2005 ..= 2019 {
        for month in 1 ..= 12 {
            for day in 1 ..= 31 {
                let date = year * 10_000 + month * 100 + day;
                dates.push(date);
            }
        }
    }

    let links:Vec<String> = dates
                .iter()
                .map(|date| format!("http://lookup-service-prod.mlb.com/json/named.ops_player_changes.bam?change_date={}", date))
                .collect()
                ;


    let data = stream(links[0..= 400].to_vec());
    println!("Got the Data!");
    let _changes_result: Vec<PlayerChanges> = data
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
                        changes: vec![],
                    }
                ))
            } 
        )
        // .inspect(|item| println!("{:?}",item))
        .collect()
        ; 

    // dbg!(&changes_result[5]);

    #[serde(from = "ChangeData")]
    #[derive(Deserialize, Debug)]
    pub struct PlayerChanges {
        // #[serde(default="default_change_data")]
        changes: Vec<Player>,
    }

    #[serde(from = "PlayerData")]
    #[derive(Deserialize, Debug)]
    pub struct Player {
        player_id: u32,
        weight: Option<u16>,
        height_feet: Option<u8>,
        height_inches: Option<u8>,
        change_year: u16,
        change_month: u8,
    }

    impl From<PlayerData> for Player {
        fn from (player: PlayerData) -> Player {
            let date_split: Vec<&str> = player.change_date.split("-").collect();

            Player {
                height_feet: player.height_feet.parse().ok(),
                height_inches: player.height_inches.parse().ok(),
                weight: player.weight.parse().ok(),
                player_id: player.player_id.parse().unwrap(),
                change_year: date_split[0].parse().unwrap(),
                change_month: date_split[1].parse().unwrap(),
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
        changes: Vec<Player>,
    }

    fn default_query_results() -> Vec<Player> {
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
        change_date: String
    }

    // dbg!(date_id[132]);

}



