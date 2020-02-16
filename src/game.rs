//! Puts all the pieces together into a useable form, stitching together the play-by-play, with the boxscore, player, venue etc.
//! 
//! 

use crate::play_by_play::{PlayEventType, Trajectory, HalfInning, Hardness, SideCode, SideDescription, PitchTypeCode, PitchTypeDescription};
use crate::boxscore::{Pos, WeatherCondition, WindDirection};
use crate::schedule::{GameType, GameTypeDescription};
use crate::venues::{SurfaceType, RoofType, TimeZone};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;



pub (crate) fn test_play_by_play () -> crate::play_by_play::Game {
    crate::play_by_play::parse_game()
}

struct GameData {
    boxscore: crate::boxscore::BoxScore,
    schedule_data: crate::schedule::Game,
    venue: crate::venues::Venue,
    venue_x_y: crate::venues::VenueXY,
    play_by_play: crate::play_by_play::Game,
}

/// GameMetaData groups together all the game level data from boxscore, schedule, live, etc. These data are not compressed and contain
/// heap allocations (Strings) so that we can easily (de)serialize the data. The size of these data should be quite manageable.
#[derive(Debug, Deserialize, Serialize)]
pub (crate) struct GameMetaData {
    game_type: GameType,
    game_type_desc: GameTypeDescription,
    game_pk: u32,
    //Date.to_string()
    game_date: String,
    game_attendance: Option<u32>,
    game_first_pitch: Option<f32>,
    game_time_zone: TimeZone,
    game_time_zone_offset: i8,
    game_weather_temp_f: Option<u8>,
    game_weather_temp_c: Option<i8>,
    game_weather_condition: Option<WeatherCondition>,
    game_wind_speed_mph: Option<u8>,
    game_wind_direction: Option<WindDirection>,
    //These MAY need to be changed to option later. TBD
    game_official_scorer_id: u32,
    game_official_scorer_name: String,
    game_primary_datacaster_id: u32,
    game_primary_datacaster_name: String,
    
    venue_id: u32,
    venue_name: String,
    venue_plate_x: f32,
    venue_plate_y: f32,
    venue_city: String,
    venue_state: String,
    venue_state_abbr: String,
    venue_time_zone: TimeZone,
    venue_time_zone_offset: i8,
    venue_capacity: Option<u32>,
    venue_surface: Option<SurfaceType>,
    venue_roof: Option<RoofType>,
    venue_left_line: Option<u16>,
    venue_left: Option<u16>,
    venue_left_center: Option<u16>,
    venue_center: Option<u16>,
    venue_right_center: Option<u16>,
    venue_right: Option<u16>,
    venue_right_line: Option<u16>,
    venue_retrosheet_id: String,

    hp_umpire_id: Option<u32>,
    hp_umpire_name: Option<String>,
    hp_umpire_dob: Option<String>,
    hp_umpire_age: Option<f32>,
    hp_umpire_height: Option<u16>,
    
}


/// Pitch is the final serializable struct that we'll export from this module. It will flatten all the at-bat level
/// data (copy) for easy use. This is intentionally de-normalized for ease of use. All data are copy, which makes this
/// very cheap. This is the efficient represenation of all pitches. For a more analytics friendly version, use the fully
/// inflated PitchCSV struct, with costly String data.
/// For ease of sorting, all counts (inning, outs balls, strikes, pitches etc. will start wit num_).
#[derive(Debug, Serialize)]
pub (crate) struct Pitch {
  
    //at_bat level meta-data
    half_inning: HalfInning,
    num_plate_appearance: u8,
    num_inning: u8,

    //Defense on the pitch. For now, use the starting lineups for everything. Update later to include subs
    //In practice, the positions should never be None. Its possible a boxscore won't be available for a game,
    //in which case we'd have no defense data. 
    catcher: Option<u32>,
    first_base: Option<u32>,
    second_base: Option<u32>,
    short_stop: Option<u32>,
    third_base: Option<u32>,
    left_field: Option<u32>,
    center_field: Option<u32>,
    right_field: Option<u32>,

    //Umpires and coaches
    hp_umpire_id: Option<u32>,
    batting_coach: Option<u32>,
    batting_manager: Option<u32>,
    pitching_coach: Option<u32>,
    pitching_manager: Option<u32>,

    //pitch level meta-data
    pitcher: u32,
    pitcher_throws: SideCode,
    pitcher_throws_desc: SideDescription,
    batter: u32,
    batter_bats: SideCode,
    batter_bats_desc: SideDescription,
    batter_pos: Pos,
    strike_zone_bottom: f32,
    strike_zone_top: f32,
    pitch_num_plate_appearance: u8,
    pitch_num_inning: u8,
    ///Pitcher specific pitch num
    pitch_num_game: u16,   
    
    //RE288 State
    balls_start: u8,
    balls_end: u8,
    strikes_start: u8,
    strikes_end: u8,
    outs_start: u8,
    outs_end: u8,
    base_value_start: u8,
    base_value_end: u8,
    runs_scored: u8,
    // is the batter responsible for the base/out/ball/strike state change?
    re_288_batter_responsible: bool,
  
    //pitch-specific data, options for MLB-level data
    in_play: u8,
    swing: u8,
    // When swing == 1 this will be Some(0) or Some (1), Else None
    swing_and_miss: Option<u8>,

    pitch_speed_start: Option<f32>,
    pitch_speed_end: Option<f32>,
    pitch_break_length: Option<f32>,
    pitch_break_y: Option<f32>,
    pitch_spin_rate: Option<f32>,
    pitch_spin_direction: Option<f32>,
    pitch_plate_time: Option<f32>,
    pitch_extension: Option<f32>,
    pitch_pixels_x: f32,
    pitch_pixels_y: f32,
    pitch_a_x: Option<f32>,
    pitch_a_y: Option<f32>,
    pitch_a_z: Option<f32>,
    pitch_pfx_x: Option<f32>,
    pitch_pfx_z: Option<f32>,
    pitch_p_x: Option<f32>,
    pitch_p_z: Option<f32>,
    pitch_v_x0: Option<f32>,
    pitch_v_y0: Option<f32>,
    pitch_v_z0: Option<f32>,
    pitch_x0: Option<f32>,
    pitch_y0: Option<f32>,
    pitch_z0: Option<f32>,
    pitch_type_code: Option<PitchTypeCode>,
    pitch_type_desc: Option<PitchTypeDescription>,

    //1B, 2B, 3B, HR, strikeout, walk for easy summing in analytical tools
    in_play_1b: u8,
    in_play_2b: u8,
    in_play_3b: u8,
    in_play_hr: u8,
    strikeout: u8,
    walk: u8,

    // hit data
    hit_data_coord_x: Option<f32>, 
    hit_data_coord_y: Option<f32>, 
    hit_data_trajectory: Option<Trajectory>,
    hit_data_contact_quality: Option<Hardness>,
    hit_data_launch_angle: Option<f32>,
    hit_data_exit_velocity: Option<f32>,
    hit_data_total_distance: Option<f32>,
    //Angle from 0 = 3B/LF Line to 90 1B/RF Line
    hit_data_spray_angle: Option<u8>,
    //distannce calculated from spray chart
    hit_data_calc_distance: Option<f32>,

}

type Pitches = Vec<Pitch>;

//Convert all the data about the game into a vector of pitches
impl From <GameData> for Pitches {

    fn from(data: GameData) -> Pitches {

        let plays = data.play_by_play.all_plays;

        // 300 should be around the size of each game. This will minimize allocations
        let mut pitches: Pitches = Vec::with_capacity(300);

        let mut home_defense = data.boxscore.home_defense;
        let mut away_defense = data.boxscore.away_defense;

        let home_players: HashMap<u32, Pos> = data.boxscore.home_players.iter().map(|p| (p.id, p.position)).collect();
        let away_players: HashMap<u32, Pos> = data.boxscore.away_players.iter().map(|p| (p.id, p.position)).collect();

        for plate_app in plays {
            // Set the initial state for the half inning
            let mut balls_start = 0u8;
            let mut balls_end = 0u8;
            let mut strikes_start = 0u8;
            let mut strikes_end = 0u8;
            let mut outs_start = 0u8;
            let mut outs_end = 0u8;
            let mut base_value_start = 0u8;
            let mut base_value_end = 0u8;

            let half_inning = plate_app.about.half_inning;
            let num_plate_appearance = plate_app.about.plate_appearance_index + 1;
            let num_inning = plate_app.about.inning_num;
            let batter = plate_app.matchup.batter_id;
            let batter_bats = plate_app.matchup.batter_bat_side_code;
            let batter_bats_desc = plate_app.matchup.batter_bat_side_desc;
            let pitcher = plate_app.matchup.pitcher_id;
            let pitcher_throws = plate_app.matchup.pitcher_pitch_hand_code;
            let pitcher_throws_desc = plate_app.matchup.pitcher_pitch_hand_desc;

            //Set the defensive and offensive players
            let (defense, players) = match half_inning {
                HalfInning::Top => (home_defense, away_players.clone()),
                HalfInning::Bottom => (away_defense, home_players.clone()),
            };

            for event in plate_app.play_events {

                match event.play_event_type {
                    PlayEventType::Action => {}
                    PlayEventType::Pickoff => {}
                    PlayEventType::Pitch => {
                        //if our event type is a pitch, we can safely unwrap the pitch_data
                        let pitch_data = event.pitch_data.unwrap();
                        
                        let (pitch_break_length, pitch_break_y, pitch_spin_rate, pitch_spin_direction) = match pitch_data.breaks {
                            Some (breaks) => 
                                (Some(breaks.break_length), Some(breaks.break_y), Some(breaks.spin_rate), Some(breaks.spin_direction)),
                            None => (None, None, None, None),
                        };

                        let (pitch_type_code, pitch_type_desc) = match event.details.pitch_type {
                            Some (pitch_type) => (Some(pitch_type.code), Some(pitch_type.description)),
                            None => (None, None),
                        };

                        let 
                            (   hit_data_coord_x, hit_data_coord_y,
                                hit_data_contact_quality,  hit_data_trajectory,
                                hit_data_exit_velocity, hit_data_launch_angle, hit_data_total_distance, 
                            ) = match event.hit_data {
                            
                            Some (hit_data) => (
                                Some(hit_data.coordinates.x), Some(hit_data.coordinates.y),
                                Some(hit_data.hardness), Some(hit_data.trajectory),
                                hit_data.launch_speed, hit_data.launch_angle, hit_data.total_distance,
                            ),
                            None => (None, None, None, None, None, None, None,),
                        };

                        pitches.push(
                            Pitch {
                                half_inning,
                                num_plate_appearance,
                                num_inning,
                                catcher: defense.catcher,
                                first_base: defense.first_base,
                                second_base: defense.second_base,
                                short_stop: defense.short_stop,
                                third_base: defense.third_base,
                                left_field: defense.left_field,
                                center_field: defense.center_field,
                                right_field: defense.right_field,
                                hp_umpire_id: data.boxscore.hp_umpire_id,

                                //TODO - Add Coach Data
                                batting_coach: None,
                                batting_manager: None,
                                pitching_coach: None,
                                pitching_manager: None,

                                pitcher,
                                pitcher_throws,
                                pitcher_throws_desc,
                                batter,
                                batter_bats,
                                batter_bats_desc,
                                batter_pos: *players.get(&batter).unwrap_or(&Pos::Bench),
                                strike_zone_top: pitch_data.strike_zone_top,
                                strike_zone_bottom: pitch_data.strike_zone_bottom,
                                
                                // TODO fix these!!!
                                pitch_num_plate_appearance: 0,
                                pitch_num_inning: 0,    
                                pitch_num_game: 0,
                                balls_start,
                                balls_end,
                                strikes_start,
                                strikes_end,
                                outs_start,
                                outs_end,
                                base_value_start,
                                base_value_end,
                                runs_scored: 0,
                                re_288_batter_responsible: true,
                                swing: 0,
                                swing_and_miss: Some(0),

                                in_play: event.details.is_in_play.unwrap().into(),

                                pitch_speed_start: pitch_data.start_speed,
                                pitch_speed_end: pitch_data.end_speed,
                                pitch_break_length,
                                pitch_break_y,
                                pitch_spin_rate,
                                pitch_spin_direction,
                                pitch_plate_time: pitch_data.plate_time,
                                pitch_extension: pitch_data.extension,
                                pitch_pixels_x: pitch_data.coordinates.x,
                                pitch_pixels_y: pitch_data.coordinates.y,
                                pitch_a_x: pitch_data.coordinates.a_x,
                                pitch_a_y: pitch_data.coordinates.a_y,
                                pitch_a_z: pitch_data.coordinates.a_z,
                                pitch_pfx_x: pitch_data.coordinates.pfx_x,
                                pitch_pfx_z: pitch_data.coordinates.pfx_z,
                                pitch_p_x: pitch_data.coordinates.p_x,
                                pitch_p_z: pitch_data.coordinates.p_z,
                                pitch_v_x0: pitch_data.coordinates.v_x0,
                                pitch_v_y0: pitch_data.coordinates.v_y0,
                                pitch_v_z0: pitch_data.coordinates.v_z0,
                                pitch_x0: pitch_data.coordinates.x0,
                                pitch_y0: pitch_data.coordinates.y0,
                                pitch_z0: pitch_data.coordinates.z0,
                                pitch_type_code,
                                pitch_type_desc,

                                // FIX THIS!!
                                in_play_1b: 0,
                                in_play_2b: 0,
                                in_play_3b: 0,
                                in_play_hr: 0,
                                strikeout: 0,
                                walk: 0,

                                hit_data_coord_x, 
                                hit_data_coord_y, 
                                hit_data_trajectory, 
                                hit_data_contact_quality, 
                                hit_data_launch_angle, 
                                hit_data_exit_velocity, 
                                hit_data_total_distance, 

                                //TODO - Add the distance and angle logic!
                                hit_data_spray_angle: Some(0),
                                hit_data_calc_distance: Some(300f32),


                            }
                        )
                    }
                }
            }
        }
        pitches
    }
}

pub fn size_of_pitch_struct() {
   
    dbg!(std::mem::size_of::<Pitch>());

}