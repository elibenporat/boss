/// # Defense Data
/// Converts pitch by pitch data into a defense database with relevant fields. This module provides one struct for the serializable defense vector,
/// as well as a From impl for the Pitch to Defense conversion. Specifically, each pitch that has an "inplay" event, is captured and then 8 records (one for each fielder type)
/// is created.
/// 
/// 

use serde::Serialize;
use crate::game::Pitch;
use crate::boxscore::Pos;
use crate::play_by_play::{Trajectory, Hardness};
use crate::schedule::GameType;

#[derive(Serialize, Debug)]
pub struct Defense {
    pub game_date: String,
    pub game_type: GameType,
    pub player: Option<u32>,
    pub player_name: Option<String>,
    pub position: Pos,
    pub batter_bats: crate::play_by_play::SideCode,
    pub batter_bats_desc: crate::play_by_play::SideDescription,
    pub batter: u32,
    pub batter_name: String,
    pub pitcher: u32,
    pub pitcher_name: String,
    pub hit_data_trajectory: Option<Trajectory>,
    pub hit_data_contact_quality: Option<Hardness>,
    pub hit_data_launch_angle: Option<f32>,
    pub hit_data_exit_velocity: Option<f32>,
    pub hit_data_total_distance: Option<f32>,
    pub hit_data_spray_angle: Option<i8>,
    pub hit_data_calc_distance: Option<f32>,
    pub sport_id: u32,
    pub sport_code: String,
    pub sport_name: String,
    pub sport_abbr: String,
    pub sport_affilliation: crate::sports::MLB,
    pub sport_level_of_play: u8,
    pub team_id: u32,
    pub team_name: String,
    pub parent_team_id: u32,
    pub parent_team_name: String,
    pub venue_id: u32,
    pub venue_name: String,
    pub venue_city: String,
    pub venue_capacity: Option<u32>,
    pub venue_surface: Option<crate::venues::SurfaceType>,
    pub venue_roof: Option<crate::venues::RoofType>,
    pub venue_left_line: Option<u16>,
    pub venue_left: Option<u16>,
    pub venue_left_center: Option<u16>,
    pub venue_center: Option<u16>,
    pub venue_right_center: Option<u16>,
    pub venue_right: Option<u16>,
    pub venue_right_line: Option<u16>,
    pub venue_retrosheet_id: String,
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
                        game_date: pitch.game_date.clone(),
                        game_type: pitch.game_type,
                        player: pitch.catcher,
                        player_name: pitch.catcher_name.clone(),
                        position: Pos::Catcher,
                        batter_bats,
                        batter_bats_desc,
                        batter,
                        batter_name: batter_name.clone(),
                        pitcher,
                        pitcher_name: pitcher_name.clone(),
                        hit_data_trajectory: pitch.hit_data_trajectory,
                        hit_data_contact_quality: pitch.hit_data_contact_quality,
                        hit_data_launch_angle: pitch.hit_data_launch_angle,
                        hit_data_exit_velocity: pitch.hit_data_exit_velocity,
                        hit_data_total_distance: pitch.hit_data_total_distance,
                        hit_data_spray_angle: pitch.hit_data_spray_angle,
                        hit_data_calc_distance: pitch.hit_data_calc_distance,
                        sport_id: pitch.sport_id,
                        sport_name: pitch.sport_name.clone(),
                        sport_code: pitch.sport_code.clone(),
                        sport_abbr: pitch.sport_abbr.clone(),
                        sport_affilliation: pitch.sport_affilliation,
                        sport_level_of_play: pitch.sport_level_of_play,
                        team_id: pitch.pitcher_team_id,
                        parent_team_id: pitch.pitcher_parent_team_id,
                        team_name: pitch.pitcher_team_name.clone(),
                        parent_team_name: pitch.pitcher_parent_team_name.clone(),
                        venue_id: pitch.venue_id,
                        venue_name: pitch.venue_name.clone(),
                        venue_city: pitch.venue_city.clone(),
                        venue_capacity: pitch.venue_capacity,
                        venue_surface: pitch.venue_surface,
                        venue_roof: pitch.venue_roof,
                        venue_left_line: pitch.venue_left_line,
                        venue_left: pitch.venue_left,
                        venue_left_center: pitch.venue_left_center,
                        venue_center: pitch.venue_center,
                        venue_right_center: pitch.venue_right_center,
                        venue_right: pitch.venue_right,
                        venue_right_line: pitch.venue_right_line,
                        venue_retrosheet_id: pitch.venue_retrosheet_id.clone(),                        
                    }],
            _ => vec![],
            
        }
    }
}