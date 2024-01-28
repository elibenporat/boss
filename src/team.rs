/// Download team metadata. We'll pull the division and other metadata from here, as well as social media channels.

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TeamData {
    pub year: u16,
    pub team: Team,
}

#[derive(Deserialize)]
pub (crate) struct TeamJson {
    pub (crate) teams: Vec<TeamDeserialize>,
}

#[derive(Deserialize, Clone)]
pub (crate) struct TeamDeserialize {
    pub (crate) id: u32,
    pub (crate) name: Option<String>,
    #[serde(rename="teamName")]
    pub (crate) team_name: Option<String>,
    pub (crate) social: Option<Social>,
    pub (crate) league: IDName,
    pub (crate) division: Option<IDName>,
    pub (crate) sport: IDName,
}

#[derive(Deserialize, Clone)]
pub (crate) struct Social {
    pub (crate) twitter: Option<Vec<String>>,
    pub (crate) facebook: Option<Vec<String>>,
    pub (crate) instagram: Option<Vec<String>>,
}

#[derive(Deserialize, Clone)]
pub (crate) struct IDName {
    pub (crate) id: Option<u32>,
    pub (crate) name: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Team {
    pub id: u32,
    pub sport_id: u32,
    pub team_city_name: String,
    pub team_name: String,
    pub division_name: Option<String>,
    pub league_name: Option<String>,
    // pub twitter: String,
    // pub facebook: String,
    // pub instagram: String,
}

impl Default for Team {
    fn default() -> Self {
        Team {
            id: 0,
            sport_id: 1,
            team_city_name: "".to_string(),
            team_name: "".to_string(),
            division_name: None,
            league_name: None,
        }
    }
}

impl From<TeamDeserialize> for Team {
    fn from (team: TeamDeserialize) -> Team {
        
        let division_name = match team.division {
            Some (div) => div.name,
            None => None,
        };
        
        Team {
            id: team.id,
            sport_id: team.sport.id.unwrap(),
            team_city_name: team.name.unwrap_or_default(),
            team_name: team.team_name.unwrap_or_default(),
            division_name,
            league_name: team.league.name,
            // twitter: team.social.unwrap_or(twitter[0].clone(),
            // facebook: team.social.facebook[0].clone(),
            // instagram: team.social.instagram[0].clone(),
        }
    }
}