/// # Defense Data
/// Converts pitch by pitch data into a defense database with relevant fields. This module provides one struct for the serializable defense vector,
/// as well as a From impl for the Pitch to Defense conversion. Specifically, each pitch that has an "inplay" event, is captured and then 8 records (one for each fielder type)
/// is created.
/// 
/// 

use serde::{Deserialize, Serialize};
use crate::game::Pitch;
use crate::boxscore::Pos;
use crate::play_by_play::{Trajectory, Hardness};
use crate::schedule::GameType;
use crate::players::{Player, SideCode, SideDescription};
use std::collections::hash_map::HashMap;
use tree_buf::{Read, Write};

#[derive(Deserialize, Serialize, Debug, Read, Write, PartialEq)]
pub struct Defense {
    pub game_date: String,
    pub game_type: GameType,
    
    pub fielder: u32,
    pub fielder_name: String,
    pub fielder_dob: String,

    pub fielder_draft_pick_number: Option<u16>,
    pub fielder_throws_code: Option<SideCode>,
    pub fielder_throws_desc: Option<SideDescription>,
    pub fielder_height_str: Option<String>,
    pub fielder_height_in: u8,
    pub fielder_weight: Option<u16>,
    pub fielder_college_name: Option<String>,
    pub fielder_birth_country: Option<String>,
    pub fielder_mlb_debut: String,

    pub fielded_by_id: Option<u32>,
    pub fielded_by_name: String,
    pub fielded_by_pos: Option<Pos>,

    pub position: Pos,
    pub outs_start: u8,
    pub outs_end: u8,
    pub balls_start: u8,
    pub strikes_start: u8,
    pub base_value_start: u8,
    pub base_value_end: u8,
    pub double_play_opp: bool,
    pub runs: u8,
    pub in_play_result: Option<crate::play_by_play::Event>,

    pub batter_bats: crate::play_by_play::SideCode,
    pub batter_bats_desc: Option<crate::play_by_play::SideDescription>,
    pub batter: u32,
    pub batter_name: String,
    pub pitcher: u32,
    pub pitcher_name: String,
    pub pitcher_throws: crate::play_by_play::SideCode,
    pub pitcher_throws_desc: Option<crate::play_by_play::SideDescription>,
    pub hit_data_trajectory: Option<Trajectory>,
    pub hit_data_contact_quality: Option<Hardness>,
    pub hit_data_launch_angle: Option<f32>,
    pub hit_data_exit_velocity: Option<f32>,
    pub hit_data_total_distance: Option<f32>,
    pub hit_data_spray_angle: Option<f32>,
    pub hit_data_calc_distance: Option<f32>,
    pub sport_id: u32,
    pub sport_code: String,
    pub sport_name: String,
    pub sport_abbr: String,
    pub sport_affilliation: crate::sports::MLB,
    pub sport_level_of_play: u8,
    pub league_name: Option<String>,
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


pub (crate) struct DefenseData <'d> {
    pub (crate) pitch: Pitch,
    pub (crate) players: &'d HashMap<u32, Player>,
}

// fn get_fielder (id: u32, players: &HashMap<u32, Player>) -> Player {
    
//     // We check in the main From impl that we have an id before we pull the data.
//     let player = players.get(&id).unwrap().to_owned();
//     player
// }

impl <'d> From<DefenseData<'d>> for Vec<Defense> {
    fn from (data: DefenseData) -> Vec<Defense> {
        let pitch = data.pitch;
        let players = data.players;

        let batter_bats = pitch.batter_bats;
        let batter_bats_desc = pitch.batter_bats_desc;
        let batter = pitch.batter;
        let batter_name = pitch.batter_name;
        let pitcher = pitch.pitcher;
        let pitcher_name = pitch.pitcher_name;

        let mut fielders: Vec<(u32, Pos)> = Vec::with_capacity(9);

        if let Some(id) = pitch.catcher_id      {fielders.push((id, Pos::Catcher))};
        if let Some(id) = pitch.first_base_id   {fielders.push((id, Pos::FirstBase))};
        if let Some(id) = pitch.second_base_id  {fielders.push((id, Pos::SecondBase))};
        if let Some(id) = pitch.third_base_id   {fielders.push((id, Pos::ThirdBase))};
        if let Some(id) = pitch.short_stop_id   {fielders.push((id, Pos::ShortStop))};
        if let Some(id) = pitch.left_field_id   {fielders.push((id, Pos::LeftField))};
        if let Some(id) = pitch.right_field_id  {fielders.push((id, Pos::RightField))};
        if let Some(id) = pitch.center_field_id {fielders.push((id, Pos::CenterField))};
        //We always have a pitcher on record
        fielders.push((pitcher, Pos::Pitcher));

        let mut fielder_data: Vec<Defense> = Vec::new();

        for fielder in fielders {
            
            // We'll ignore all records where we don't have fielder metadata.
            // This should be somewhat rare. So far this only occurs for the
            // the following ids: 580899, 581659
            let meta = match players.get(&fielder.0) {
                Some (player) => player.to_owned(),
                None => {
                    println!("Couldn't find metadata for: {}", &fielder.0);
                    return vec![];
                },
            };

            
            let fielder_dob = match meta.birth_date {
                Some(date) => date.to_string(),
                None => "".to_string(),
            };

            let fielder_mlb_debut = match  meta.mlb_debut_date {
                Some (date) => date.to_string(),
                None => "".to_string(),
            };

            fielder_data.push(
                Defense {
                    game_date: pitch.game_date.clone(),
                    game_type: pitch.game_type,
                    fielder: fielder.0,
                    fielder_name: meta.name,
                    fielder_dob,
                    fielder_draft_pick_number: meta.draft_pick_number,
                    fielder_throws_code: meta.throws_code,
                    fielder_throws_desc: meta.throws_description,
                    fielder_height_str: meta.height_str,
                    fielder_height_in: meta.height_in,
                    fielder_weight: meta.weight,
                    fielder_college_name: meta.college_name,
                    fielder_birth_country: meta.birth_country,
                    fielder_mlb_debut,

                    fielded_by_id: pitch.fielded_by_id,
                    fielded_by_name: pitch.fielded_by_name.clone(),
                    fielded_by_pos: pitch.fielded_by_pos,

                    outs_start: pitch.outs_start,
                    outs_end: pitch.outs_end,
                    balls_start: pitch.balls_start,
                    strikes_start: pitch.strikes_start,
                    base_value_start: pitch.base_value_start,
                    base_value_end: pitch.base_value_end,
                    double_play_opp: pitch.double_play_opportunity,
                    runs: pitch.runs_scored,
                    in_play_result: pitch.in_play_result,

                    position: fielder.1,
                    batter_bats,
                    batter_bats_desc,
                    batter,
                    batter_name: batter_name.clone(),
                    pitcher,
                    pitcher_throws: pitch.pitcher_throws,
                    pitcher_throws_desc: pitch.pitcher_throws_desc,
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
                    league_name: pitch.league_name_home.clone(),
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
                }

            )
        }

        // Only return the data if we have a ball in play. Could arguably save a little work
        // by checking for this earlier, but I think it's cleaner this way.
        match pitch.in_play {
            1 =>  fielder_data,
            _ => vec![],
            
        }
    }
}