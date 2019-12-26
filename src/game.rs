//! Puts all the pieces together into a useable form, stitching together the play-by-play, with the boxscore, player, venue etc.
//! 
//! 

use crate::play_by_play::{Trajectory, HalfInning, Hardness, SideCode, SideDescription, PitchTypeCode, PitchTypeDescription};
use crate::boxscore::{Pos};


/// Pitch is the final serializable struct that we'll export from this module. It will flatten all the at-bat level
/// data (copy) for easy use. This is intentionally de-normalized for ease of use. All data are copy, which makes this
/// very cheap. This is the efficient represenation of all pitches. For a more analytics friendly version, use the fully
/// inflated PitchCSV struct, with costly String data.
/// For ease of sorting, all counts (inning, outs balls, strikes, pitches etc. will start wit num_).
#[derive(Debug, Serialize)]
struct Pitch {
  
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
    hp_umpire: u32,
    batting_coach: u32,
    batting_manager: u32,
    pitching_coach: u32,
    pitching_manager: u32,

    //pitch level meta-data
    pitcher: u32,
    pitcher_throws: SideCode,
    pitcher_throws_desc: SideDescription,
    batter: u32,
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
    is_in_play: bool,
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
    in_play_1B: u8,
    in_play_2B: u8,
    in_play_3B: u8,
    in_play_HR: u8,
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
