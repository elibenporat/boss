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




#[derive(Clone, Debug)]
pub struct VecMetaDataInputs {
    pub boxscore:   Vec<BoxScoreData>,
    pub venue:      Vec<VenueData>,
    pub venue_x_y:  Vec<VenueXY>,
    pub schedule:   Vec<GameMetaData>,
    // pub coaches:    Vec<CoachData>,
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
    // pub coaches:        HashMap<GamePK,             CoachData>,
    pub teams:          HashMap<(ID, Year),         Team>,
    pub players:        HashMap<ID,                 Player>,
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

        // let coaches: HashMap<u32, CoachData> = meta.coaches
        //     .clone()
        //     .into_iter()
        //     .map (|c| (c.game_pk, c))
        //     .collect()
        //     ;

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
            // coaches,
            teams,
            players,
            re_288_default,
        }
    }
}