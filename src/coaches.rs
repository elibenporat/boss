//! Parse the coach data for a particular day. We pull coach data for each game, even though
//! 


use serde::{Deserialize, Serialize};

pub fn test_coaches() {

    let test = get_coaches(COACHES.to_string());
    dbg!(test);

}

pub (crate) fn get_coaches (coach_data: String) -> Coaches {
    
    serde_json::from_str(&coach_data).unwrap()
    
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CoachData {
  pub game_pk: u32,
  pub home_coaches: Coaches,
  pub away_coaches: Coaches,
}

impl Default for CoachData {
  fn default() -> Self {
    CoachData {
      game_pk: 0,
      home_coaches: Coaches {
        batting_coach: None,
        pitching_coach: None,
        manager: None,
      },
      away_coaches: Coaches {
        batting_coach: None,
        pitching_coach: None,
        manager: None,
      },
    }
  }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Coaches {
    pub batting_coach: Option<u32>,
    pub pitching_coach: Option<u32>,
    pub manager: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Roster {
    pub (crate) roster: Vec<Coach>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Coach {
  pub(crate) person: Person,
  pub(crate) job: String,
  #[serde(rename="jobId")]
  pub(crate)job_id: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Person {
  pub(crate) id: u32,
  #[serde(rename="fullName")]
  pub(crate) full_name: String,
}

impl From <Roster> for Coaches {
    fn from (roster: Roster) -> Coaches {
        let batting_coach = roster.roster.iter()
            .filter (|coach| 
                coach.job_id.as_str() == "COAT" ||
                coach.job_id.as_str() == "COAH" ||
                coach.job_id.as_str() == "COHI"
            )
            .map (|coach| coach.person.id)
            .nth(0);
        
        let pitching_coach = roster.roster.iter()
            .filter (|coach| 
                coach.job_id.as_str() == "COAP" ||
                coach.job_id.as_str() == "COPI"
            )
            .map (|coach| coach.person.id)
            .nth(0);
        
        let manager = roster.roster.iter()
            .filter (|coach| 
                coach.job_id.as_str() == "MNGR" ||
                coach.job_id.as_str() == "NTRM"
            )
            .map (|coach| coach.person.id)
            .nth(0);
        

        Coaches {
            batting_coach,
            pitching_coach,
            manager,
        }
        

    }
}


const COACHES: &str = r#"{
    "copyright" : "Copyright 2019 MLB Advanced Media, L.P.  Use of any content on this page acknowledges agreement to the terms posted here http://gdx.mlb.com/components/copyright.txt",
    "roster" : [ {
      "person" : {
        "id" : 118942,
        "fullName" : "Bob Melvin",
        "link" : "/api/v1/people/118942"
      },
      "jerseyNumber" : "6",
      "job" : "Manager",
      "jobId" : "MNGR"
    }, {
      "person" : {
        "id" : 134342,
        "fullName" : "Ryan Christenson",
        "link" : "/api/v1/people/134342"
      },
      "jerseyNumber" : "29",
      "job" : "Bench Coach",
      "jobId" : "COAB"
    }, {
      "person" : {
        "id" : 470252,
        "fullName" : "Darren Bush",
        "link" : "/api/v1/people/470252"
      },
      "jerseyNumber" : "51",
      "job" : "Hitting Coach",
      "jobId" : "COAT"
    }, {
      "person" : {
        "id" : 110119,
        "fullName" : "Mike Aldrete",
        "link" : "/api/v1/people/110119"
      },
      "jerseyNumber" : "17",
      "job" : "Assistant Hitting Coach",
      "jobId" : "COAA"
    }, {
      "person" : {
        "id" : 491913,
        "fullName" : "Scott Emerson",
        "link" : "/api/v1/people/491913"
      },
      "jerseyNumber" : "14",
      "job" : "Pitching Coach",
      "jobId" : "COAP"
    }, {
      "person" : {
        "id" : 120349,
        "fullName" : "Al Pedrique",
        "link" : "/api/v1/people/120349"
      },
      "jerseyNumber" : "41",
      "job" : "First Base Coach",
      "jobId" : "COA1"
    }, {
      "person" : {
        "id" : 124326,
        "fullName" : "Matt Williams",
        "link" : "/api/v1/people/124326"
      },
      "jerseyNumber" : "4",
      "job" : "Third Base Coach",
      "jobId" : "COA3"
    }, {
      "person" : {
        "id" : 116534,
        "fullName" : "Marcus Jensen",
        "link" : "/api/v1/people/116534"
      },
      "jerseyNumber" : "59",
      "job" : "Bullpen Coach",
      "jobId" : "COAU"
    }, {
      "person" : {
        "id" : 117276,
        "fullName" : "Mark Kotsay",
        "link" : "/api/v1/people/117276"
      },
      "jerseyNumber" : "7",
      "job" : "Quality Control Coach",
      "jobId" : "QUAC"
    }, {
      "person" : {
        "id" : 623951,
        "fullName" : "Jeremy Dowdy",
        "link" : "/api/v1/people/623951"
      },
      "jerseyNumber" : "90",
      "job" : "Bullpen Catcher",
      "jobId" : "BCAT"
    }, {
      "person" : {
        "id" : 543653,
        "fullName" : "Philip Pohl",
        "link" : "/api/v1/people/543653"
      },
      "jerseyNumber" : "88",
      "job" : "Bullpen Catcher",
      "jobId" : "BCAT"
    } ],
    "link" : "/api/v1/teams/133/coaches",
    "teamId" : 133,
    "rosterType" : "coach"
  }"#;