/// Converts all the various metadata components that are specific to each game into one giant struct. This is important enough to
/// warrant its own separate module. Initially, this will just create one giant, unbreakable metadata struct, however we hope to eventually allow for
/// optional serialization so that the final output can be more efficient. We'll likely use the skip_serializing_if serde feature for that.
/// 
/// Rather than importing the namespaces into this module, we explicitly use crate::* to make it clear where each field is being drawn from. 
/// 


use std::collections::HashMap;
use crate::schedule::GameMetaData;
use crate::boxscore::{BoxScore, BoxScoreData};
use crate::venues::{Venue, VenueXY, VenueData};
use crate::coaches::CoachData;
use crate::team::{Team, TeamData};
use crate::players::Player;
use crate::feed_live::FeedData;




#[derive(Clone, Debug)]
pub struct VecMetaDataInputs {
    pub boxscore:   Vec<BoxScoreData>,
    pub venue:      Vec<VenueData>,
    pub venue_x_y:  Vec<VenueXY>,
    pub schedule:   Vec<GameMetaData>,
    pub coaches:    Vec<CoachData>,
    pub feed_data:  Vec<FeedData>,
    pub teams:      Vec<TeamData>,
    pub players:    Vec<Player>,
}

///u32
type GamePK = u32;
///u32
type ID = u32;
///u16
type Year = u16;

pub struct MetaData {
    pub schedule:       HashMap<GamePK,             GameMetaData>,
    pub boxscore:       HashMap<GamePK,             BoxScore>,
    pub venue:          HashMap<(ID, Year),         Venue>,
    pub venue_x_y:      HashMap<ID,                 VenueXY>,
    pub coaches:        HashMap<GamePK,             CoachData>,
    pub teams:          HashMap<(ID, Year),         Team>,
    pub players:        HashMap<ID,                 Player>,
    pub feed:           HashMap<GamePK,             FeedData>,
    pub re_288_default: HashMap<(u8, u8, u8, u8),   f32>,
}

// Converts all metadata into Hashmaps that the play by play data can use.
impl From<VecMetaDataInputs> for MetaData {

    fn from (meta: VecMetaDataInputs) -> MetaData {

        //Schedule Meta indexed by gamePk
        let schedule: HashMap<GamePK, GameMetaData> = meta.schedule
            .clone()
            .into_iter()
            .map(|s| (s.game_pk, s))
            .collect()
            ;

        //Boxscore Meta indexed by gamePk
        let boxscore: HashMap<u32, BoxScore> = meta.boxscore
            .clone()
            .into_iter()
            .map(|b| (b.game_pk, b.boxscore_data))
            .collect()
            ;

        //Venue Meta indexed by (venue_id, year)
        let venue: HashMap<(u32, u16), Venue> = meta.venue
            .clone()
            .into_iter()
            .map (|v| ((v.venue.id, v.year), v.venue))
            .collect()
            ;

        let venue_x_y: HashMap<u32, VenueXY> = meta.venue_x_y
            .clone()
            .into_iter()
            .map (|v| (v.id, v))
            .collect()
            ;

        let coaches: HashMap<u32, CoachData> = meta.coaches
            .clone()
            .into_iter()
            .map (|c| (c.game_pk, c))
            .collect()
            ;

        let teams: HashMap<(u32, u16), Team> = meta.teams
            .clone()
            .into_iter()
            .map (|t| ((t.team.id, t.year), t.team))
            .collect()
            ;

        let players: HashMap<u32, Player> = meta.players
            .clone()
            .into_iter()
            .map (|p| (p.id, p))
            .collect()
            ;
        
        let feed: HashMap<u32, FeedData> = meta.feed_data
            .clone()
            .into_iter()
            .map (|f| (f.game_pk, f))
            .collect()
            ;

        let re_288_default: HashMap<(u8, u8, u8, u8), f32> =
            crate::run_expectancy::RE288_DEFAULT.iter()
            .map (|re| ((re.balls, re.strikes, re.base_value, re.outs), re.run_expectancy))
            .collect()
            ;

        MetaData {
            schedule,
            boxscore,
            venue,
            venue_x_y,
            coaches,
            teams,
            players,
            feed,
            re_288_default,
        }
    }
}






// impl From<VecMetaDataInputs> for Vec<MetaDataInputs> {
//     fn from (vecs: VecMetaDataInputs) -> VecMetaDataInputs{

//         //Schedule Meta indexed by gamePk
//         let sched_map: HashMap<u32, crate::schedule::GameMetaData> = vecs.schedule
//             .clone()
//             .into_iter()
//             .map(|s| (s.game_pk, s))
//             .collect()
//             ;

//         //Boxscore Meta indexed by gamePk
//         let box_map: HashMap<u32, crate::boxscore::BoxScore> = vecs.boxscore
//             .clone()
//             .into_iter()
//             .map(|b| (b.game_pk, b.boxscore_data))
//             .collect()
//             ;
        

//     todo!()
//     }
// }

// struct MetaDataInputs {
//     boxscore: crate::boxscore::BoxScore,
//     venue: crate::venues::Venue,
//     venue_x_y: crate::venues::VenueXY,
//     schedule: crate::schedule::GameMetaData,
//     coaches: crate::coaches::CoachData,
//     feed_data: crate::feed_live::FeedData,
//     teams: crate::team::TeamData,
// }

// #[derive(Debug, Serialize)]
// pub struct MetaData {

//     // Fields relevant to the level of play
//     pub sport_id: u32,
//     pub sport_code: String,
//     pub sport_name: String,
//     pub sport_abbr: String,
//     pub sport_affilliation: crate::sports::MLB,
//     pub sport_level_of_play: u8,
    
//     // Game Level MetaData
//     pub game_pk: u32,
//     pub game_type: crate::schedule::GameType,
//     pub game_type_desc: crate::schedule::GameTypeDescription,
//     pub game_date: Date,
//     pub game_status: crate::schedule::AbstractGameState,
    
//     // Venue Metadata
//     pub venue_id: u32,
//     pub venue_home_plate_x: Option<f32>,
//     pub venue_home_plate_y: Option<f32>,
//     pub venue_name: String,
//     pub venue_city: String,
//     pub venue_state: String,
//     pub venue_state_abbr: String,
//     pub venue_time_zone: crate::venues::TimeZone,
//     pub venue_time_zone_offset: i8,
//     pub venue_capacity: Option<u32>,
//     pub venue_surface: Option<crate::venues::SurfaceType>,
//     pub venue_roof: Option<crate::venues::RoofType>,
//     pub venue_left_line: Option<u16>,
//     pub venue_left: Option<u16>,
//     pub venue_left_center: Option<u16>,
//     pub venue_center: Option<u16>,
//     pub venue_right_center: Option<u16>,
//     pub venue_right: Option<u16>,
//     pub venue_right_line: Option<u16>,
//     pub venue_retrosheet_id: String,
//     pub venue_latitude: Option<f32>,
//     pub venue_longitude: Option<f32>,

//     //Boxscore MetaData
//     pub game_attendance: Option<u32>,
//     pub game_first_pitch: Option<f32>,
//     pub game_weather_temp_f: Option<u8>,
//     pub game_weather_temp_c: Option<i8>,
//     pub game_weather_condition: Option<crate::boxscore::WeatherCondition>,
//     pub game_wind_speed_mph: Option<u8>,
//     pub game_wind_direction: Option<crate::boxscore::WindDirection>,   
    
//     pub team_id_home: u32,
//     pub team_id_away: u32,
//     pub league_id_home: Option<u32>,
//     pub league_id_away: Option<u32>,
//     pub sport_id_home: u32,
//     pub sport_id_away: u32,
//     pub parent_team_id_home: u32,
//     pub parent_team_id_away: u32,

//     pub team_name_home: String,
//     pub team_name_away: String,
//     pub parent_team_name_home: String,
//     pub parent_team_name_away: String,

//     pub league_name_home: String,

//     pub hp_umpire_id: Option<u32>,

//     pub home_manager_id: Option<u32>,
//     pub home_batting_coach_id: Option<u32>,
//     pub home_pitching_coach_id: Option<u32>,
    
//     pub away_manager_id: Option<u32>,
//     pub away_batting_coach_id: Option<u32>,
//     pub away_pitching_coach_id: Option<u32>,

//     pub official_scorer_id: Option<u32>,
//     pub official_scorer_name: Option<String>,
//     pub primary_datacaster_id: Option<u32>,
//     pub primary_datacaster_name: Option<String>,

// }

// /// Converts all the various game metadata into one flattened struct
// impl From<MetaDataInputs> for MetaData {
//     fn from (meta: MetaDataInputs) -> MetaData {
        
//         let sport_meta_data: BTreeMap<u32, crate::sports::Sport> = crate::sports::SPORTS
//             .iter()
//             .map(|sport| (sport.id, *sport))
//             .collect()
//             ;
        
//         let sport_id = meta.schedule.sport_id;

//         // It should be impossible for the sport_meta_data to return none, so we want to panic
//         // if that happens for some reason.
//         let sport = match sport_meta_data.get(&sport_id) {
//             Some (sport_data) => sport_data,
//             None => panic!(),
//         };
        
//         MetaData {
//             sport_id,
//             sport_code: sport.code.into(),
//             sport_name: sport.name.into(),
//             sport_abbr: sport.abbr.into(),
//             sport_affilliation: sport.affiliation,
//             sport_level_of_play: sport.level_of_play_rank,

//             game_pk: meta.schedule.game_pk,
//             game_type: meta.schedule.game_type,
//             game_type_desc: meta.schedule.game_type_desc,
//             game_date: meta.schedule.game_date,
//             game_status: meta.schedule.game_status,

//             venue_id: meta.schedule.game_venue_id,
//             venue_home_plate_x: meta.venue_x_y.x,
//             venue_home_plate_y: meta.venue_x_y.y,
//             venue_name: meta.venue.venue_name,
//             venue_city: meta.venue.venue_city,
//             venue_state: meta.venue.venue_state,
//             venue_state_abbr: meta.venue.venue_state_abbr,
//             venue_time_zone: meta.venue.venue_time_zone,
//             venue_time_zone_offset: meta.venue.venue_time_zone_offset,
//             venue_capacity:meta.venue.venue_capacity,
//             venue_surface: meta.venue.venue_surface,
//             venue_roof: meta.venue.venue_roof,
//             venue_left_line: meta.venue.venue_left_line,
//             venue_left: meta.venue.venue_left,
//             venue_left_center: meta.venue.venue_left_center,
//             venue_center: meta.venue.venue_center,
//             venue_right_center: meta.venue.venue_right_center,
//             venue_right: meta.venue.venue_right,
//             venue_right_line: meta.venue.venue_right_line,
//             venue_retrosheet_id: meta.venue.venue_retrosheet_id,
//             venue_latitude: meta.venue.venue_latitude,
//             venue_longitude: meta.venue.venue_longitude,

//             game_attendance: meta.boxscore.attendance,
//             game_first_pitch: meta.boxscore.first_pitch,
//             game_weather_condition: meta.boxscore.game_weather_condition,
//             game_weather_temp_f: meta.boxscore.game_weather_temp_f,
//             game_weather_temp_c: meta.boxscore.game_weather_temp_c,
//             game_wind_speed_mph: meta.boxscore.game_wind_speed_mph,
//             game_wind_direction: meta.boxscore.game_wind_direction,
//             team_id_home: meta.boxscore.home_team_id,
//             team_id_away: meta.boxscore.away_team_id,
//             league_id_home: meta.boxscore.home_league_id,
//             league_id_away: meta.boxscore.away_league_id,
//             sport_id_home: meta.boxscore.home_sport_id,
//             sport_id_away: meta.boxscore.away_sport_id,
//             parent_team_id_home: meta.boxscore.home_parent_team_id,
//             parent_team_id_away: meta.boxscore.away_parent_team_id,
//             hp_umpire_id: meta.boxscore.hp_umpire_id,

//             home_manager_id: meta.coaches.home_coaches.manager,
//             home_pitching_coach_id: meta.coaches.home_coaches.pitching_coach,
//             home_batting_coach_id: meta.coaches.home_coaches.batting_coach,
//             away_manager_id: meta.coaches.away_coaches.manager,
//             away_pitching_coach_id: meta.coaches.away_coaches.pitching_coach,
//             away_batting_coach_id: meta.coaches.away_coaches.batting_coach,

//             official_scorer_id: meta.feed_data.official_scorer_id,
//             official_scorer_name: meta.feed_data.official_scorer_name,
//             primary_datacaster_id: meta.feed_data.primary_datacaster_id,
//             primary_datacaster_name: meta.feed_data.primary_datacaster_name,

//         }
//     }
// }