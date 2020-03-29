/// # Defense Data
/// Converts pitch by pitch data into a defense database with relevant fields. This module provides one struct for the serializable defense vector,
/// as well as a From impl for the Pitch to Defense conversion. Specifically, each pitch that has an "inplay" event, is captured and then 8 records (one for each fielder type)
/// is created.
/// 
/// 

use serde::Serialize;
use crate::game::Pitch;
use crate::boxscore::Pos;

#[derive(Serialize, Debug)]
struct Defense {
    player: Option<u32>,
    player_name: Option<String>,
    position: Pos,
    batter_bats: crate::play_by_play::SideCode,
    batter_bats_desc: crate::play_by_play::SideDescription,
    batter: u32,
    batter_name: String,
    pitcher: u32,
    pitcher_name: String,
}

impl From<Pitch> for Vec<Defense> {
    fn from (pitch: Pitch) -> Vec<Defense> {
        let batter_bats = pitch.batter_bats;
        let batter_bats_desc = pitch.batter_bats_desc;
        let batter = pitch.batter;
        let batter_name = pitch.batter_name;
        let pitcher = pitch.pitcher;
        let pitcher_name = pitch.pitcher_name;

        match pitch.in_play {
            1 =>  vec![
                    Defense {
                        player: pitch.catcher,
                        player_name: pitch.catcher_name.clone(),
                        position: Pos::Catcher,
                        batter_bats,
                        batter_bats_desc,
                        batter,
                        batter_name: batter_name.clone(),
                        pitcher,
                        pitcher_name: pitcher_name.clone(),
                    }],
            _ => vec![],
            
        }
    }
}