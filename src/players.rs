/// The players module contains all the metadata for players. We're going to build up a lot of metadata here, including awards, injury history and draft history.
/// We'll create 3 separate files, one for the player's metadata, and one each for the injuries and awards. Player data will need to be refreshed monthly in order
/// to update all the history data, as well as get their most recent weight.
/// 



use serde::{Deserialize, Serialize};
use std::collections::hash_map::HashMap;
use crate::utils::*;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub birth_city: Option<String>,
    pub birth_state_province: Option<String>,
    pub birth_country: Option<String>,
    pub height_str: Option<String>,
    pub height_in: u8,
    pub weight: Option<u16>,
    pub highschool_city: Option<String>,
    pub highschool_prov_state: Option<String>,
    pub college_name: Option<String>,
    pub bat_side_code: Option<SideCode>,
    pub bat_side_description: Option<SideDescription>,
    pub throws_code: Option<SideCode>,
    pub throws_description: Option<SideDescription>,
    pub birth_date: Option<Date>,
    pub draft_school_name: Option<String>,
    pub draft_year: Option<u16>,
    pub draft_pick_round: Option<String>,
    pub draft_pick_number: Option<u16>,
    pub fangraphs_id: Option<String>,
    pub retrosheet_id: Option<String>,
    pub twitter_id: Option<String>,
    pub facebook_id: Option<String>,
    pub instagram_id: Option<String>,
    pub mlb_debut_date: Option<Date>,
}

impl From <PlayerDeserialize> for Player {
    fn from (player: PlayerDeserialize) -> Player {
        
        let xrefs: HashMap<String, String> = player.clone().xref_ids.unwrap_or(vec![]).into_iter()
            .map(|xref| (xref.xref_type, xref.xref_id))
            .collect()
            ;
        let fangraphs_id = xrefs.get("fangraphs").cloned();
        let retrosheet_id = xrefs.get("retrosheet").cloned();
        let facebook_id = xrefs.get("facebook").cloned();
        let twitter_id = xrefs.get("twitter").cloned();
        let instagram_id = xrefs.get("instagram").cloned();

        let height_in: u8 = player.height.clone().unwrap_or("".to_string()).split("'").into_iter()
            .enumerate()
            .map (|(n, h)| h.replace("\"","").trim().parse().unwrap_or(0) * 12u8.pow((1-n) as u32))
            .sum()
            ;
        
        let (highschool_city, highschool_prov_state) = match player.clone().education.highschools {
            Some (hs) => (Some(hs[0].city.clone().unwrap_or("".to_string())), Some(hs[0].state.clone().unwrap_or("".to_string()))),
            None => (None, None),
        };

        let college_name = match player.clone().education.colleges {
            Some (college) => Some(college[0].name.clone().unwrap_or("".to_string())),
            None => None,
        };

        let drafts: Vec<Draft> = player.clone().drafts.unwrap_or(vec![])
            .into_iter()
            .filter(|d| d.year.parse::<u16>().unwrap_or(0) == player.clone().draft_year.unwrap_or(0))
            .collect()
            ;

        let (draft_school_name, draft_pick_number, draft_pick_round) = match drafts.len() {
            0 => (None, None, None),
            _ => (Some(drafts[0].school.name.clone().unwrap_or("".to_string())),Some(drafts[0].pick_number),Some(drafts[0].pick_round.clone())),
        };
        
        let mlb_debut_date = match player.mlb_debut_date {
            Some (debut) => Some(debut.into()),
            None => None,
        };

        let birth_date = match player.birth_date {
            Some (date) => Some(date.into()),
            None => None,
        };

        let (bat_side_code, bat_side_description) = match player.bat_side {
            Some (code) => (Some(code.code), code.description),
            _ => (None, None),

        };

        let (throws_code, throws_description) = match player.throws {
            Some (code) => (Some(code.code), code.description),
            _ => (None, None),

        };

        Player {
            id: player.id,
            name: player.name,
            birth_city: player.birth_city,
            birth_state_province: player.birth_state_province,
            birth_country: player.birth_country,
            birth_date,
            height_in,
            height_str: player.height,
            weight: player.weight,

            highschool_city,
            highschool_prov_state,
            college_name,

            draft_year: player.draft_year,
            draft_pick_number,
            draft_pick_round,
            draft_school_name,

            fangraphs_id,
            retrosheet_id,
            twitter_id,
            facebook_id,
            instagram_id,

            mlb_debut_date,
            bat_side_code,
            bat_side_description,

            throws_code,
            throws_description,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub (crate) struct People {
    pub (crate) people: Vec<PlayerDeserialize>,
}

#[derive(Deserialize, Debug, Clone)]
pub (crate) struct PlayerDeserialize {
    pub (crate) id: u32,
    #[serde(alias="fullName")]
    pub (crate) name: String,
    #[serde(alias="birthCity")]
    pub (crate) birth_city: Option<String>,
    #[serde(alias="birthStateProvince", alias="birthState")]
    pub (crate) birth_state_province: Option<String>,
    #[serde(alias="birthCountry")]
    pub (crate) birth_country: Option<String>,
    pub (crate) height: Option<String>,
    pub (crate) weight: Option<u16>,
    pub (crate) education: Education,
    #[serde(alias="batSide")]
    pub (crate) bat_side: Option<Side>,
    #[serde(alias="pitchHand")]
    pub (crate) throws: Option<Side>,
    #[serde(alias="birthDate")]
    pub (crate) birth_date: Option<String>,
    pub (crate) drafts: Option<Vec<Draft>>,
    #[serde(alias="xrefIds")]
    pub (crate) xref_ids: Option<Vec<IDType>>,
    #[serde(alias="mlbDebutDate")]
    pub (crate) mlb_debut_date: Option<String>,
    #[serde(alias="draftYear")]
    pub (crate) draft_year: Option<u16>,
}

#[serde(rename_all="camelCase")]
#[derive(Deserialize, Debug, Clone)]
pub (crate) struct IDType {
    pub (crate) xref_id: String,
    pub (crate) xref_type: String,
}

#[serde(rename_all="camelCase")]
#[derive(Deserialize, Debug, Clone)]
pub (crate) struct Draft {
    pub (crate) pick_round: String,
    pub (crate) pick_number: u16,
    pub (crate) school: School,
    pub (crate) year: String,
}

#[derive(Deserialize, Debug, Clone)]
pub (crate) struct School {
    pub (crate) name: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub (crate) struct Education {
    pub (crate) highschools: Option<Vec<Highschool>>,
    pub (crate) colleges: Option<Vec<College>>,
}

#[derive(Deserialize, Debug, Clone)]
pub (crate) struct Highschool {
    pub (crate) city: Option<String>,
    pub (crate) state: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub (crate) struct College {
    pub (crate) name: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub (crate) struct Side {
    pub (crate) code: SideCode,
    pub (crate) description: Option<SideDescription>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum SideCode {
    R,
    L,
    S,
    B,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum SideDescription {
    Right,
    Left,
    Switch,
    Either,
}



// Player Awards. 
// We want to see if the number of player awards a player gets in the minors is predictive of future success. I'm not sure yet
// what the best way to aggregate this is, so we'll seprate them out first before baking them into the data.


// Player Transactions
// Transaction data are critical as they contain injury history. This is essential if we want to include injury history in a projection system.
// In order to limit the number of times we pull a player's data, we'll need two separate pulls - one that updates all missing players without transaction data, as well
// as one that pulls in ALL the players, including transaction and award data, minus players who have retired.