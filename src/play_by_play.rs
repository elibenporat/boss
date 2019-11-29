//! API Bindings for the MLB Play by Play API hosted at http://statsapi.mlb.com/api/v1/game/{game_pk}/playByPlay
//! All data are subject to MLB copyright
//! 
//! There are differences with respect to MLB-level data as compared to Minor League data. This means that for fields that are MLB specific
//! we'll have to wrap them in Option<T> to signify they won't always be there. We want a unified data set, so we'll only create on set of structs
//! 
//! 
//! 


// use isahc::prelude::*;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub (crate) struct Game {
    all_plays: Vec<AllPlays>,
}

#[derive(Deserialize, Debug)]
pub (crate) struct AllPlays {
    result: Result,
}

#[derive(Deserialize, Debug)]
pub (crate) struct Result {
    #[serde(rename="type")]
    result_type: ResultType,
    #[serde(rename="event")]
    plate_appearance_result: PlateAppearanceResult,
}


#[derive(Deserialize, Debug)]
#[serde(field_identifier, rename_all="camelCase")]
pub (crate) enum ResultType {
    AtBat,
}


/// PlateAppearanceResult stores all the possible plate appearance results. Wherever possible, we'll convert text
/// into enums, avoiding lifetime issues and increasing memory efficiency. Serde does all the heavy lifting in the
/// background.
#[derive(Debug, Deserialize)]
#[serde(field_identifier)]
pub (crate) enum PlateAppearanceResult {
    #[serde(alias = "Batter Interference")]
    BatterInterference,
    #[serde(alias = "Bunt Ground Out", alias = "Bunt Groundout")]
    BuntGroundOut,
    #[serde(alias = "Bunt Pop Out", alias = "Bunt Popout")]
    BuntPopOut,
    #[serde(alias = "Catcher Interference")]
    CatcherInterference,
    Double,
    #[serde(alias = "Double Play")]
    DoublePlay,
    #[serde(alias = "Fan Interference")]
    FanInterference,
    #[serde(alias = "Field Error")]
    FieldError,
    #[serde(alias = "Field Out", alias="Fieldout")]
    FieldOut,
    #[serde(alias = "Fielders Choice", alias = "Fielders Choice Out")]
    FieldersChoice,
    #[serde(alias = "Fly Out", alias = "Flyout")]
    FlyOut,
    #[serde(alias = "Force Out", alias = "Forceout")]
    ForceOut,
    #[serde(alias = "Ground Out", alias = "Groundout")]
    GroundOut,
    #[serde(alias = "Grounded into DP")]
    GroundedIntoDoublePlay,
    #[serde(alias = "Hit By Pitch")]
    HitByPitch,
    #[serde(alias = "Home Run")]
    HomeRun,
    #[serde(alias = "Intent Walk", alias = "Intentional Walk")]
    IntentionalWalk,
    #[serde(alias = "Line Out", alias="Lineout")]
    LineOut,
    #[serde(alias = "Pop Out", alias = "Popout")]
    PopOut,
    #[serde(alias = "Runner Out")]
    RunnerOut,
    #[serde(alias = "Sac Bunt")]
    SacBunt,
    #[serde(alias = "Sac Fly")]
    SacFly,
    #[serde(alias = "Sac Fly DP")]
    SacFlyDoublePlay,
    #[serde(alias = "Sacrifice Bunt DP")]
    SacrificeBuntDoublePlay,
    Single,
    #[serde(alias = "Strikeout", alias = "Strikeout - DP", alias = "Strikeout - TP")]
    StrikeOut,
    Triple,
    #[serde(alias = "Triple Play")]
    TriplePlay,
    Walk,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub (crate) struct PlateAppearance {
    #[serde(rename="type")]
    result_type: ResultType,
    event: PlateAppearanceResult,
    event_type: String,
}


pub fn parse_test_data () {

    let test_parse: Game = serde_json::from_str(TEST_DATA).unwrap();
    dbg!(test_parse);

}

const TEST_DATA: &str = r#"{
  "copyright" : "Copyright 2019 MLB Advanced Media, L.P.  Use of any content on this page acknowledges agreement to the terms posted here http://gdx.mlb.com/components/copyright.txt",
  "allPlays" : [ {
    "result" : {
      "type" : "atBat",
      "event" : "Pop Out",
      "eventType" : "field_out",
      "description" : "Wader Perez pops out to first baseman Geancarlo Mendez.",
      "rbi" : 0,
      "awayScore" : 0,
      "homeScore" : 0
    },
    "about" : {
      "atBatIndex" : 0,
      "halfInning" : "top",
      "inning" : 1,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 1
    },
    "matchup" : {
      "batter" : {
        "id" : 542458,
        "fullName" : "Wader Perez",
        "link" : "/api/v1/people/542458"
      },
      "batSide" : {
        "code" : "L",
        "description" : "Left"
      },
      "pitcher" : {
        "id" : 516710,
        "fullName" : "Siulman Lebron",
        "link" : "/api/v1/people/516710"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_LHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 1
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : null,
        "runner" : {
          "id" : 542458,
          "fullName" : "Wader Perez",
          "link" : "/api/v1/people/542458"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 542479,
          "link" : "/api/v1/people/542479"
        },
        "position" : {
          "code" : "3",
          "name" : "First Base",
          "type" : "Infielder",
          "abbreviation" : "1B"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 85.84,
          "y" : 131.24
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "popup",
        "hardness" : "medium",
        "location" : "3",
        "coordinates" : {
          "coordX" : 142.57,
          "coordY" : 175.7
        }
      },
      "index" : 0,
      "playId" : "02401986-0016-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 0
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Groundout",
      "eventType" : "field_out",
      "description" : "Alexander Castellano grounds out, second baseman Carlos Valenzuela to first baseman Geancarlo Mendez.",
      "rbi" : 0,
      "awayScore" : 0,
      "homeScore" : 0
    },
    "about" : {
      "atBatIndex" : 1,
      "halfInning" : "top",
      "inning" : 1,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 542463,
        "fullName" : "Alex Castellano",
        "link" : "/api/v1/people/542463"
      },
      "batSide" : {
        "code" : "L",
        "description" : "Left"
      },
      "pitcher" : {
        "id" : 516710,
        "fullName" : "Siulman Lebron",
        "link" : "/api/v1/people/516710"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_LHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 2
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : null,
        "runner" : {
          "id" : 542463,
          "fullName" : "Alex Castellano",
          "link" : "/api/v1/people/542463"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 542477,
          "link" : "/api/v1/people/542477"
        },
        "position" : {
          "code" : "4",
          "name" : "Second Base",
          "type" : "Infielder",
          "abbreviation" : "2B"
        },
        "credit" : "f_assist"
      }, {
        "player" : {
          "id" : 542479,
          "link" : "/api/v1/people/542479"
        },
        "position" : {
          "code" : "3",
          "name" : "First Base",
          "type" : "Infielder",
          "abbreviation" : "1B"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 90.99,
          "y" : 126.93
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "ground_ball",
        "hardness" : "medium",
        "location" : "4",
        "coordinates" : {
          "coordX" : 132.53,
          "coordY" : 168.67
        }
      },
      "index" : 0,
      "playId" : "02401986-0026-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 1
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Groundout",
      "eventType" : "field_out",
      "description" : "Juan B Cabrera grounds out, second baseman Carlos Valenzuela to first baseman Geancarlo Mendez.",
      "rbi" : 0,
      "awayScore" : 0,
      "homeScore" : 0
    },
    "about" : {
      "atBatIndex" : 2,
      "halfInning" : "top",
      "inning" : 1,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 3
    },
    "matchup" : {
      "batter" : {
        "id" : 516868,
        "fullName" : "Juan B Cabrera",
        "link" : "/api/v1/people/516868"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 516710,
        "fullName" : "Siulman Lebron",
        "link" : "/api/v1/people/516710"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 3
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : null,
        "runner" : {
          "id" : 516868,
          "fullName" : "Juan B Cabrera",
          "link" : "/api/v1/people/516868"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 542477,
          "link" : "/api/v1/people/542477"
        },
        "position" : {
          "code" : "4",
          "name" : "Second Base",
          "type" : "Infielder",
          "abbreviation" : "2B"
        },
        "credit" : "f_assist"
      }, {
        "player" : {
          "id" : 542479,
          "link" : "/api/v1/people/542479"
        },
        "position" : {
          "code" : "3",
          "name" : "First Base",
          "type" : "Infielder",
          "abbreviation" : "1B"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 103.0,
          "y" : 116.57
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "ground_ball",
        "hardness" : "medium",
        "location" : "4",
        "coordinates" : {
          "coordX" : 135.54,
          "coordY" : 167.67
        }
      },
      "index" : 0,
      "playId" : "02401986-0036-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 2
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Strikeout",
      "eventType" : "strikeout",
      "description" : "Jonathan Villan strikes out swinging.",
      "rbi" : 0,
      "awayScore" : 0,
      "homeScore" : 0
    },
    "about" : {
      "atBatIndex" : 3,
      "halfInning" : "bottom",
      "inning" : 1,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 14
    },
    "count" : {
      "balls" : 0,
      "strikes" : 3,
      "outs" : 1
    },
    "matchup" : {
      "batter" : {
        "id" : 542340,
        "fullName" : "Jonathan Villar",
        "link" : "/api/v1/people/542340"
      },
      "batSide" : {
        "code" : "L",
        "description" : "Left"
      },
      "pitcher" : {
        "id" : 542203,
        "fullName" : "Angel De Jesus",
        "link" : "/api/v1/people/542203"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_LHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0, 1, 2 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 1
      },
      "details" : {
        "event" : "Strikeout",
        "eventType" : "strikeout",
        "movementReason" : null,
        "runner" : {
          "id" : 542340,
          "fullName" : "Jonathan Villar",
          "link" : "/api/v1/people/542340"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 2
      },
      "credits" : [ {
        "player" : {
          "id" : 542233,
          "link" : "/api/v1/people/542233"
        },
        "position" : {
          "code" : "2",
          "name" : "Catcher",
          "type" : "Catcher",
          "abbreviation" : "C"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 1
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 97.85,
          "y" : 126.06
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0046-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 2
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 96.14,
          "y" : 128.65
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0046-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 3
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 96.14,
          "y" : 119.16
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0046-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 3
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Strikeout",
      "eventType" : "strikeout",
      "description" : "Carlos Valenzuela strikes out swinging, catcher Audris Perez to first baseman Marcos Martinez.",
      "rbi" : 0,
      "awayScore" : 0,
      "homeScore" : 0
    },
    "about" : {
      "atBatIndex" : 4,
      "halfInning" : "bottom",
      "inning" : 1,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 14
    },
    "count" : {
      "balls" : 0,
      "strikes" : 3,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 542477,
        "fullName" : "Carlos Valenzuela",
        "link" : "/api/v1/people/542477"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542203,
        "fullName" : "Angel De Jesus",
        "link" : "/api/v1/people/542203"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0, 1, 2 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 2
      },
      "details" : {
        "event" : "Strikeout",
        "eventType" : "strikeout",
        "movementReason" : null,
        "runner" : {
          "id" : 542477,
          "fullName" : "Carlos Valenzuela",
          "link" : "/api/v1/people/542477"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 2
      },
      "credits" : [ {
        "player" : {
          "id" : 542233,
          "link" : "/api/v1/people/542233"
        },
        "position" : {
          "code" : "2",
          "name" : "Catcher",
          "type" : "Catcher",
          "abbreviation" : "C"
        },
        "credit" : "f_assist"
      }, {
        "player" : {
          "id" : 516658,
          "link" : "/api/v1/people/516658"
        },
        "position" : {
          "code" : "3",
          "name" : "First Base",
          "type" : "Infielder",
          "abbreviation" : "1B"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 1
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 96.14,
          "y" : 123.47
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0056-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 2
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 95.28,
          "y" : 109.66
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0056-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 3
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 96.14,
          "y" : 117.43
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0056-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 4
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Flyout",
      "eventType" : "field_out",
      "description" : "Rudney Balentien flies out to right fielder Alexander Castellano.",
      "rbi" : 0,
      "awayScore" : 0,
      "homeScore" : 0
    },
    "about" : {
      "atBatIndex" : 5,
      "halfInning" : "bottom",
      "inning" : 1,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 3
    },
    "matchup" : {
      "batter" : {
        "id" : 516680,
        "fullName" : "Rudney Balentien",
        "link" : "/api/v1/people/516680"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542203,
        "fullName" : "Angel De Jesus",
        "link" : "/api/v1/people/542203"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 3
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : null,
        "runner" : {
          "id" : 516680,
          "fullName" : "Rudney Balentien",
          "link" : "/api/v1/people/516680"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 542463,
          "link" : "/api/v1/people/542463"
        },
        "position" : {
          "code" : "9",
          "name" : "Outfielder",
          "type" : "Outfielder",
          "abbreviation" : "RF"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 79.83,
          "y" : 126.06
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "fly_ball",
        "hardness" : "medium",
        "location" : "9",
        "coordinates" : {
          "coordX" : 168.67,
          "coordY" : 109.44
        }
      },
      "index" : 0,
      "playId" : "02401986-0066-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 5
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Strikeout",
      "eventType" : "strikeout",
      "description" : "Audris Perez strikes out swinging.",
      "rbi" : 0,
      "awayScore" : 0,
      "homeScore" : 0
    },
    "about" : {
      "atBatIndex" : 6,
      "halfInning" : "top",
      "inning" : 2,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 14
    },
    "count" : {
      "balls" : 0,
      "strikes" : 3,
      "outs" : 1
    },
    "matchup" : {
      "batter" : {
        "id" : 542233,
        "fullName" : "Audry Perez",
        "link" : "/api/v1/people/542233"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 516710,
        "fullName" : "Siulman Lebron",
        "link" : "/api/v1/people/516710"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0, 1, 2 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 1
      },
      "details" : {
        "event" : "Strikeout",
        "eventType" : "strikeout",
        "movementReason" : null,
        "runner" : {
          "id" : 542233,
          "fullName" : "Audry Perez",
          "link" : "/api/v1/people/542233"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 2
      },
      "credits" : [ {
        "player" : {
          "id" : 501517,
          "link" : "/api/v1/people/501517"
        },
        "position" : {
          "code" : "2",
          "name" : "Catcher",
          "type" : "Catcher",
          "abbreviation" : "C"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 1
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 100.43,
          "y" : 126.93
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0076-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 2
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 101.29,
          "y" : 121.75
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0076-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 3
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 94.42,
          "y" : 117.43
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0076-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 6
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Single",
      "eventType" : "single",
      "description" : "Luis Pimentel singles on a ground ball to left fielder Jose Trinidad.",
      "rbi" : 0,
      "awayScore" : 0,
      "homeScore" : 0
    },
    "about" : {
      "atBatIndex" : 7,
      "halfInning" : "top",
      "inning" : 2,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 33
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 1
    },
    "matchup" : {
      "batter" : {
        "id" : 540938,
        "fullName" : "Luis Pimentel",
        "link" : "/api/v1/people/540938"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 516710,
        "fullName" : "Siulman Lebron",
        "link" : "/api/v1/people/516710"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Men_On"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : null,
        "runner" : {
          "id" : 540938,
          "fullName" : "Luis Pimentel",
          "link" : "/api/v1/people/540938"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 542339,
          "link" : "/api/v1/people/542339"
        },
        "position" : {
          "code" : "7",
          "name" : "Outfielder",
          "type" : "Outfielder",
          "abbreviation" : "LF"
        },
        "credit" : "f_fielded_ball"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "D",
          "description" : "Hit Into Play - No Out(s)"
        },
        "description" : "In play, no out",
        "code" : "D",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 97.0,
          "y" : 128.65
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "ground_ball",
        "hardness" : "medium",
        "location" : "7",
        "coordinates" : {
          "coordX" : 69.28,
          "coordY" : 128.51
        }
      },
      "index" : 0,
      "playId" : "02401986-0086-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 7
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Groundout",
      "eventType" : "field_out",
      "description" : "Santo Sandoval grounds out, second baseman Carlos Valenzuela to first baseman Geancarlo Mendez.    Luis Pimentel to 2nd.",
      "rbi" : 0,
      "awayScore" : 0,
      "homeScore" : 0
    },
    "about" : {
      "atBatIndex" : 8,
      "halfInning" : "top",
      "inning" : 2,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 542468,
        "fullName" : "Santo Sandoval",
        "link" : "/api/v1/people/542468"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 516710,
        "fullName" : "Siulman Lebron",
        "link" : "/api/v1/people/516710"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "RISP"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0, 1 ],
    "runners" : [ {
      "movement" : {
        "start" : "1B",
        "end" : "2B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : "r_adv_play",
        "runner" : {
          "id" : 540938,
          "fullName" : "Luis Pimentel",
          "link" : "/api/v1/people/540938"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      }
    }, {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 2
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : null,
        "runner" : {
          "id" : 542468,
          "fullName" : "Santo Sandoval",
          "link" : "/api/v1/people/542468"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 542477,
          "link" : "/api/v1/people/542477"
        },
        "position" : {
          "code" : "4",
          "name" : "Second Base",
          "type" : "Infielder",
          "abbreviation" : "2B"
        },
        "credit" : "f_assist"
      }, {
        "player" : {
          "id" : 542479,
          "link" : "/api/v1/people/542479"
        },
        "position" : {
          "code" : "3",
          "name" : "First Base",
          "type" : "Infielder",
          "abbreviation" : "1B"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 98.71,
          "y" : 121.75
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "ground_ball",
        "hardness" : "medium",
        "location" : "4",
        "coordinates" : {
          "coordX" : 132.53,
          "coordY" : 166.67
        }
      },
      "index" : 0,
      "playId" : "02401986-0096-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 8
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Field Error",
      "eventType" : "field_error",
      "description" : "Bernardo Villar reaches on throwing error by first baseman Geancarlo Mendez.    Luis Pimentel scores.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 0
    },
    "about" : {
      "atBatIndex" : 9,
      "halfInning" : "top",
      "inning" : 2,
      "isComplete" : true,
      "isScoringPlay" : true,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 542248,
        "fullName" : "Bernardo Villar",
        "link" : "/api/v1/people/542248"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 516710,
        "fullName" : "Siulman Lebron",
        "link" : "/api/v1/people/516710"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Men_On"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0, 1 ],
    "runners" : [ {
      "movement" : {
        "start" : "2B",
        "end" : "score",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Field Error",
        "eventType" : "field_error",
        "movementReason" : "r_adv_play",
        "runner" : {
          "id" : 540938,
          "fullName" : "Luis Pimentel",
          "link" : "/api/v1/people/540938"
        },
        "responsiblePitcher" : {
          "id" : 516710,
          "link" : "/api/v1/people/516710"
        },
        "isScoringEvent" : true,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : true,
        "playIndex" : 0
      }
    }, {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Field Error",
        "eventType" : "field_error",
        "movementReason" : "r_adv_play",
        "runner" : {
          "id" : 542248,
          "fullName" : "Bernardo Villar",
          "link" : "/api/v1/people/542248"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 542479,
          "link" : "/api/v1/people/542479"
        },
        "position" : {
          "code" : "3",
          "name" : "First Base",
          "type" : "Infielder",
          "abbreviation" : "1B"
        },
        "credit" : "f_throwing_error"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "E",
          "description" : "Hit Into Play - Run(s)"
        },
        "description" : "In play, run(s)",
        "code" : "E",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 92.7,
          "y" : 112.25
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "ground_ball",
        "hardness" : "medium",
        "location" : "3",
        "coordinates" : {
          "coordX" : 145.58,
          "coordY" : 175.7
        }
      },
      "index" : 0,
      "playId" : "02401986-0106-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 9
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Groundout",
      "eventType" : "field_out",
      "description" : "Roberto Reyes grounds out, first baseman Geancarlo Mendez to pitcher Siulman Lebron.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 0
    },
    "about" : {
      "atBatIndex" : 10,
      "halfInning" : "top",
      "inning" : 2,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 3
    },
    "matchup" : {
      "batter" : {
        "id" : 542465,
        "fullName" : "Roberto Reyes",
        "link" : "/api/v1/people/542465"
      },
      "batSide" : {
        "code" : "L",
        "description" : "Left"
      },
      "pitcher" : {
        "id" : 516710,
        "fullName" : "Siulman Lebron",
        "link" : "/api/v1/people/516710"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_LHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 3
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : null,
        "runner" : {
          "id" : 542465,
          "fullName" : "Roberto Reyes",
          "link" : "/api/v1/people/542465"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 542479,
          "link" : "/api/v1/people/542479"
        },
        "position" : {
          "code" : "3",
          "name" : "First Base",
          "type" : "Infielder",
          "abbreviation" : "1B"
        },
        "credit" : "f_assist"
      }, {
        "player" : {
          "id" : 516710,
          "link" : "/api/v1/people/516710"
        },
        "position" : {
          "code" : "1",
          "name" : "Pitcher",
          "type" : "Pitcher",
          "abbreviation" : "P"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 90.99,
          "y" : 120.88
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "ground_ball",
        "hardness" : "medium",
        "location" : "3",
        "coordinates" : {
          "coordX" : 142.57,
          "coordY" : 174.7
        }
      },
      "index" : 0,
      "playId" : "02401986-0116-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 10
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Walk",
      "eventType" : "walk",
      "description" : "Pedro Aguilar walks.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 0
    },
    "about" : {
      "atBatIndex" : 11,
      "halfInning" : "bottom",
      "inning" : 2,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 4,
      "strikes" : 0,
      "outs" : 0
    },
    "matchup" : {
      "batter" : {
        "id" : 516729,
        "fullName" : "Pedro Aguilar",
        "link" : "/api/v1/people/516729"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542203,
        "fullName" : "Angel De Jesus",
        "link" : "/api/v1/people/542203"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Men_On"
      }
    },
    "pitchIndex" : [ 0, 1, 2, 3 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Walk",
        "eventType" : "walk",
        "movementReason" : null,
        "runner" : {
          "id" : 516729,
          "fullName" : "Pedro Aguilar",
          "link" : "/api/v1/people/516729"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 3
      },
      "credits" : [ ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 51.5,
          "y" : 120.88
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0126-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 2,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 64.38,
          "y" : 105.34
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0126-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 3,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 51.5,
          "y" : 107.93
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0126-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 4,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 54.94,
          "y" : 107.07
        },
        "breaks" : { }
      },
      "index" : 3,
      "playId" : "02401986-0126-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 11
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Walk",
      "eventType" : "walk",
      "description" : "Geancarlo Mendez walks.    Pedro Aguilar to 2nd.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 0
    },
    "about" : {
      "atBatIndex" : 12,
      "halfInning" : "bottom",
      "inning" : 2,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 4,
      "strikes" : 0,
      "outs" : 0
    },
    "matchup" : {
      "batter" : {
        "id" : 542479,
        "fullName" : "Geancarlo Mendez",
        "link" : "/api/v1/people/542479"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542203,
        "fullName" : "Angel De Jesus",
        "link" : "/api/v1/people/542203"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "RISP"
      }
    },
    "pitchIndex" : [ 0, 1, 2, 3 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0, 1 ],
    "runners" : [ {
      "movement" : {
        "start" : "1B",
        "end" : "2B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Walk",
        "eventType" : "walk",
        "movementReason" : "r_adv_force",
        "runner" : {
          "id" : 516729,
          "fullName" : "Pedro Aguilar",
          "link" : "/api/v1/people/516729"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 3
      }
    }, {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Walk",
        "eventType" : "walk",
        "movementReason" : null,
        "runner" : {
          "id" : 542479,
          "fullName" : "Geancarlo Mendez",
          "link" : "/api/v1/people/542479"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 3
      },
      "credits" : [ ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 55.79,
          "y" : 120.02
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0136-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 2,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 64.38,
          "y" : 107.07
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0136-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 3,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 52.36,
          "y" : 101.02
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0136-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 4,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 61.8,
          "y" : 110.52
        },
        "breaks" : { }
      },
      "index" : 3,
      "playId" : "02401986-0136-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 12
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Walk",
      "eventType" : "walk",
      "description" : "Emmanuel Checo walks.    Pedro Aguilar to 3rd.    Geancarlo Mendez to 2nd.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 0
    },
    "about" : {
      "atBatIndex" : 13,
      "halfInning" : "bottom",
      "inning" : 2,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 4,
      "strikes" : 0,
      "outs" : 0
    },
    "matchup" : {
      "batter" : {
        "id" : 501517,
        "fullName" : "Emmanuel Checo",
        "link" : "/api/v1/people/501517"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542203,
        "fullName" : "Angel De Jesus",
        "link" : "/api/v1/people/542203"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Loaded"
      }
    },
    "pitchIndex" : [ 0, 1, 2, 3 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0, 1, 2 ],
    "runners" : [ {
      "movement" : {
        "start" : "2B",
        "end" : "3B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Walk",
        "eventType" : "walk",
        "movementReason" : "r_adv_force",
        "runner" : {
          "id" : 516729,
          "fullName" : "Pedro Aguilar",
          "link" : "/api/v1/people/516729"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 3
      }
    }, {
      "movement" : {
        "start" : "1B",
        "end" : "2B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Walk",
        "eventType" : "walk",
        "movementReason" : "r_adv_force",
        "runner" : {
          "id" : 542479,
          "fullName" : "Geancarlo Mendez",
          "link" : "/api/v1/people/542479"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 3
      }
    }, {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Walk",
        "eventType" : "walk",
        "movementReason" : null,
        "runner" : {
          "id" : 501517,
          "fullName" : "Emmanuel Checo",
          "link" : "/api/v1/people/501517"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 3
      },
      "credits" : [ ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 45.49,
          "y" : 116.57
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0146-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 2,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 66.95,
          "y" : 99.3
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0146-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 3,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 62.66,
          "y" : 120.02
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0146-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 4,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 60.94,
          "y" : 113.11
        },
        "breaks" : { }
      },
      "index" : 3,
      "playId" : "02401986-0146-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 13
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Triple",
      "eventType" : "triple",
      "description" : "Luis Paulino triples (2) on a fly ball to center fielder Roberto Reyes.    Pedro Aguilar scores.    Geancarlo Mendez scores.    Emmanuel Checo scores.",
      "rbi" : 3,
      "awayScore" : 1,
      "homeScore" : 3
    },
    "about" : {
      "atBatIndex" : 14,
      "halfInning" : "bottom",
      "inning" : 2,
      "isComplete" : true,
      "isScoringPlay" : true,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 72
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 0
    },
    "matchup" : {
      "batter" : {
        "id" : 516760,
        "fullName" : "Luis Paulino",
        "link" : "/api/v1/people/516760"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542203,
        "fullName" : "Angel De Jesus",
        "link" : "/api/v1/people/542203"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "RISP"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0, 1, 2, 3 ],
    "runners" : [ {
      "movement" : {
        "start" : "3B",
        "end" : "score",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Triple",
        "eventType" : "triple",
        "movementReason" : "r_adv_force",
        "runner" : {
          "id" : 516729,
          "fullName" : "Pedro Aguilar",
          "link" : "/api/v1/people/516729"
        },
        "responsiblePitcher" : {
          "id" : 542203,
          "link" : "/api/v1/people/542203"
        },
        "isScoringEvent" : true,
        "rbi" : true,
        "earned" : true,
        "teamUnearned" : false,
        "playIndex" : 0
      }
    }, {
      "movement" : {
        "start" : "2B",
        "end" : "score",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Triple",
        "eventType" : "triple",
        "movementReason" : "r_adv_force",
        "runner" : {
          "id" : 542479,
          "fullName" : "Geancarlo Mendez",
          "link" : "/api/v1/people/542479"
        },
        "responsiblePitcher" : {
          "id" : 542203,
          "link" : "/api/v1/people/542203"
        },
        "isScoringEvent" : true,
        "rbi" : true,
        "earned" : true,
        "teamUnearned" : false,
        "playIndex" : 0
      }
    }, {
      "movement" : {
        "start" : "1B",
        "end" : "score",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Triple",
        "eventType" : "triple",
        "movementReason" : "r_adv_force",
        "runner" : {
          "id" : 501517,
          "fullName" : "Emmanuel Checo",
          "link" : "/api/v1/people/501517"
        },
        "responsiblePitcher" : {
          "id" : 542203,
          "link" : "/api/v1/people/542203"
        },
        "isScoringEvent" : true,
        "rbi" : true,
        "earned" : true,
        "teamUnearned" : false,
        "playIndex" : 0
      }
    }, {
      "movement" : {
        "start" : null,
        "end" : "3B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Triple",
        "eventType" : "triple",
        "movementReason" : null,
        "runner" : {
          "id" : 516760,
          "fullName" : "Luis Paulino",
          "link" : "/api/v1/people/516760"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 542465,
          "link" : "/api/v1/people/542465"
        },
        "position" : {
          "code" : "8",
          "name" : "Outfielder",
          "type" : "Outfielder",
          "abbreviation" : "CF"
        },
        "credit" : "f_fielded_ball"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "E",
          "description" : "Hit Into Play - Run(s)"
        },
        "description" : "In play, run(s)",
        "code" : "E",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 97.85,
          "y" : 115.7
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "fly_ball",
        "hardness" : "medium",
        "location" : "8",
        "coordinates" : {
          "coordX" : 124.5,
          "coordY" : 58.23
        }
      },
      "index" : 0,
      "playId" : "02401986-0156-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 14
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Strikeout",
      "eventType" : "strikeout",
      "description" : "Jose Trinidad strikes out swinging.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 3
    },
    "about" : {
      "atBatIndex" : 15,
      "halfInning" : "bottom",
      "inning" : 2,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 14
    },
    "count" : {
      "balls" : 0,
      "strikes" : 3,
      "outs" : 1
    },
    "matchup" : {
      "batter" : {
        "id" : 542339,
        "fullName" : "Jose Trinidad",
        "link" : "/api/v1/people/542339"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542203,
        "fullName" : "Angel De Jesus",
        "link" : "/api/v1/people/542203"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "RISP"
      }
    },
    "pitchIndex" : [ 0, 1, 2 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 1
      },
      "details" : {
        "event" : "Strikeout",
        "eventType" : "strikeout",
        "movementReason" : null,
        "runner" : {
          "id" : 542339,
          "fullName" : "Jose Trinidad",
          "link" : "/api/v1/people/542339"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 2
      },
      "credits" : [ {
        "player" : {
          "id" : 542233,
          "link" : "/api/v1/people/542233"
        },
        "position" : {
          "code" : "2",
          "name" : "Catcher",
          "type" : "Catcher",
          "abbreviation" : "C"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 1
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 95.28,
          "y" : 117.43
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0166-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "C",
          "description" : "Strike - Called"
        },
        "description" : "Called Strike",
        "code" : "C",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 2
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 98.71,
          "y" : 121.75
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0166-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 3
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 103.86,
          "y" : 116.57
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0166-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 15
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Pop Out",
      "eventType" : "field_out",
      "description" : "Carlos Best pops out to third baseman Luis Pimentel.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 3
    },
    "about" : {
      "atBatIndex" : 16,
      "halfInning" : "bottom",
      "inning" : 2,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 542758,
        "fullName" : "Carlos Best",
        "link" : "/api/v1/people/542758"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542203,
        "fullName" : "Angel De Jesus",
        "link" : "/api/v1/people/542203"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "RISP"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 2
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : null,
        "runner" : {
          "id" : 542758,
          "fullName" : "Carlos Best",
          "link" : "/api/v1/people/542758"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 540938,
          "link" : "/api/v1/people/540938"
        },
        "position" : {
          "code" : "5",
          "name" : "Third Base",
          "type" : "Infielder",
          "abbreviation" : "3B"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 93.56,
          "y" : 124.34
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "popup",
        "hardness" : "medium",
        "location" : "5",
        "coordinates" : {
          "coordX" : 87.35,
          "coordY" : 159.64
        }
      },
      "index" : 0,
      "playId" : "02401986-0176-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 16
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Field Error",
      "eventType" : "field_error",
      "description" : "Jonathan Villan reaches on throwing error by shortstop Juan B Cabrera.    Luis Paulino scores.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 17,
      "halfInning" : "bottom",
      "inning" : 2,
      "isComplete" : true,
      "isScoringPlay" : true,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 542340,
        "fullName" : "Jonathan Villar",
        "link" : "/api/v1/people/542340"
      },
      "batSide" : {
        "code" : "L",
        "description" : "Left"
      },
      "pitcher" : {
        "id" : 542203,
        "fullName" : "Angel De Jesus",
        "link" : "/api/v1/people/542203"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_LHB",
        "menOnBase" : "Men_On"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0, 1 ],
    "runners" : [ {
      "movement" : {
        "start" : "3B",
        "end" : "score",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Field Error",
        "eventType" : "field_error",
        "movementReason" : "r_adv_play",
        "runner" : {
          "id" : 516760,
          "fullName" : "Luis Paulino",
          "link" : "/api/v1/people/516760"
        },
        "responsiblePitcher" : {
          "id" : 542203,
          "link" : "/api/v1/people/542203"
        },
        "isScoringEvent" : true,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : true,
        "playIndex" : 0
      }
    }, {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Field Error",
        "eventType" : "field_error",
        "movementReason" : "r_adv_play",
        "runner" : {
          "id" : 542340,
          "fullName" : "Jonathan Villar",
          "link" : "/api/v1/people/542340"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 516868,
          "link" : "/api/v1/people/516868"
        },
        "position" : {
          "code" : "6",
          "name" : "Shortstop",
          "type" : "Infielder",
          "abbreviation" : "SS"
        },
        "credit" : "f_throwing_error"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "E",
          "description" : "Hit Into Play - Run(s)"
        },
        "description" : "In play, run(s)",
        "code" : "E",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 100.43,
          "y" : 120.02
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "ground_ball",
        "hardness" : "medium",
        "location" : "6",
        "coordinates" : {
          "coordX" : 106.43,
          "coordY" : 168.67
        }
      },
      "index" : 0,
      "playId" : "02401986-0186-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 17
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Pop Out",
      "eventType" : "field_out",
      "description" : "Carlos Valenzuela pops out to second baseman Wader Perez.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 18,
      "halfInning" : "bottom",
      "inning" : 2,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 1,
      "strikes" : 0,
      "outs" : 3
    },
    "matchup" : {
      "batter" : {
        "id" : 542477,
        "fullName" : "Carlos Valenzuela",
        "link" : "/api/v1/people/542477"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542203,
        "fullName" : "Angel De Jesus",
        "link" : "/api/v1/people/542203"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0, 2 ],
    "actionIndex" : [ 1 ],
    "runnerIndex" : [ 2 ],
    "runners" : [ {
      "movement" : {
        "start" : "1B",
        "end" : "2B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Stolen Base 2B",
        "eventType" : "stolen_base_2b",
        "movementReason" : "r_stolen_base_2b",
        "runner" : {
          "id" : 542340,
          "fullName" : "Jonathan Villar",
          "link" : "/api/v1/people/542340"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 1
      },
      "credits" : [ ]
    }, {
      "movement" : {
        "start" : "2B",
        "end" : "3B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Error",
        "eventType" : "error",
        "movementReason" : "r_adv_play",
        "runner" : {
          "id" : 542340,
          "fullName" : "Jonathan Villar",
          "link" : "/api/v1/people/542340"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 1
      },
      "credits" : [ {
        "player" : {
          "id" : 542233,
          "link" : "/api/v1/people/542233"
        },
        "position" : {
          "code" : "2",
          "name" : "Catcher",
          "type" : "Catcher",
          "abbreviation" : "C"
        },
        "credit" : "f_throwing_error"
      } ]
    }, {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 3
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : null,
        "runner" : {
          "id" : 542477,
          "fullName" : "Carlos Valenzuela",
          "link" : "/api/v1/people/542477"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 2
      },
      "credits" : [ {
        "player" : {
          "id" : 542458,
          "link" : "/api/v1/people/542458"
        },
        "position" : {
          "code" : "4",
          "name" : "Second Base",
          "type" : "Infielder",
          "abbreviation" : "2B"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 46.35,
          "y" : 116.57
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0196-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "description" : "With Carlos Valenzuela batting, Jonathan Villan steals (12) 2nd base,  .  Jonathan Villan advances to 3rd, on throwing error by catcher Audris Perez.",
        "event" : "Stolen Base 2B",
        "eventType" : "stolen_base_2b",
        "awayScore" : 1,
        "homeScore" : 4,
        "isScoringPlay" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0,
        "outs" : 2
      },
      "index" : 1,
      "isPitch" : false,
      "type" : "action",
      "player" : {
        "id" : 542340,
        "link" : "/api/v1/people/542340"
      }
    }, {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 97.0,
          "y" : 113.98
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "popup",
        "hardness" : "medium",
        "location" : "4",
        "coordinates" : {
          "coordX" : 127.51,
          "coordY" : 141.57
        }
      },
      "index" : 2,
      "playId" : "02401986-0196-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 18
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Groundout",
      "eventType" : "field_out",
      "description" : "Marcos Martinez grounds out, third baseman Luis Paulino to first baseman Geancarlo Mendez.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 19,
      "halfInning" : "top",
      "inning" : 3,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 1
    },
    "matchup" : {
      "batter" : {
        "id" : 516658,
        "fullName" : "Marcos Martinez",
        "link" : "/api/v1/people/516658"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 516710,
        "fullName" : "Siulman Lebron",
        "link" : "/api/v1/people/516710"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 1
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : null,
        "runner" : {
          "id" : 516658,
          "fullName" : "Marcos Martinez",
          "link" : "/api/v1/people/516658"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 516760,
          "link" : "/api/v1/people/516760"
        },
        "position" : {
          "code" : "5",
          "name" : "Third Base",
          "type" : "Infielder",
          "abbreviation" : "3B"
        },
        "credit" : "f_assist"
      }, {
        "player" : {
          "id" : 542479,
          "link" : "/api/v1/people/542479"
        },
        "position" : {
          "code" : "3",
          "name" : "First Base",
          "type" : "Infielder",
          "abbreviation" : "1B"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 98.71,
          "y" : 125.2
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "ground_ball",
        "hardness" : "medium",
        "location" : "5",
        "coordinates" : {
          "coordX" : 100.4,
          "coordY" : 177.71
        }
      },
      "index" : 0,
      "playId" : "02401986-0206-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 19
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Single",
      "eventType" : "single",
      "description" : "Wader Perez singles on a ground ball to right fielder Carlos Best.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 20,
      "halfInning" : "top",
      "inning" : 3,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 33
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 1
    },
    "matchup" : {
      "batter" : {
        "id" : 542458,
        "fullName" : "Wader Perez",
        "link" : "/api/v1/people/542458"
      },
      "batSide" : {
        "code" : "L",
        "description" : "Left"
      },
      "pitcher" : {
        "id" : 516710,
        "fullName" : "Siulman Lebron",
        "link" : "/api/v1/people/516710"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_LHB",
        "menOnBase" : "Men_On"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : null,
        "runner" : {
          "id" : 542458,
          "fullName" : "Wader Perez",
          "link" : "/api/v1/people/542458"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 542758,
          "link" : "/api/v1/people/542758"
        },
        "position" : {
          "code" : "9",
          "name" : "Outfielder",
          "type" : "Outfielder",
          "abbreviation" : "RF"
        },
        "credit" : "f_fielded_ball"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "D",
          "description" : "Hit Into Play - No Out(s)"
        },
        "description" : "In play, no out",
        "code" : "D",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 97.85,
          "y" : 124.34
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "ground_ball",
        "hardness" : "medium",
        "location" : "9",
        "coordinates" : {
          "coordX" : 151.61,
          "coordY" : 115.46
        }
      },
      "index" : 0,
      "playId" : "02401986-0216-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 20
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Pop Out",
      "eventType" : "field_out",
      "description" : "Alexander Castellano pops out to third baseman Luis Paulino in foul territory.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 21,
      "halfInning" : "top",
      "inning" : 3,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 542463,
        "fullName" : "Alex Castellano",
        "link" : "/api/v1/people/542463"
      },
      "batSide" : {
        "code" : "L",
        "description" : "Left"
      },
      "pitcher" : {
        "id" : 516710,
        "fullName" : "Siulman Lebron",
        "link" : "/api/v1/people/516710"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_LHB",
        "menOnBase" : "Men_On"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 2
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : null,
        "runner" : {
          "id" : 542463,
          "fullName" : "Alex Castellano",
          "link" : "/api/v1/people/542463"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 516760,
          "link" : "/api/v1/people/516760"
        },
        "position" : {
          "code" : "5",
          "name" : "Third Base",
          "type" : "Infielder",
          "abbreviation" : "3B"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 97.0,
          "y" : 114.84
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "popup",
        "hardness" : "medium",
        "location" : "5",
        "coordinates" : {
          "coordX" : 94.38,
          "coordY" : 192.77
        }
      },
      "index" : 0,
      "playId" : "02401986-0226-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 21
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Forceout",
      "eventType" : "force_out",
      "description" : "Juan B Cabrera grounds into a force out, second baseman Carlos Valenzuela to shortstop Jonathan Villan.    Wader Perez out at 2nd.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 22,
      "halfInning" : "top",
      "inning" : 3,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 3
    },
    "matchup" : {
      "batter" : {
        "id" : 516868,
        "fullName" : "Juan B Cabrera",
        "link" : "/api/v1/people/516868"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 516710,
        "fullName" : "Siulman Lebron",
        "link" : "/api/v1/people/516710"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0, 1 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Forceout",
        "eventType" : "force_out",
        "movementReason" : null,
        "runner" : {
          "id" : 516868,
          "fullName" : "Juan B Cabrera",
          "link" : "/api/v1/people/516868"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ ]
    }, {
      "movement" : {
        "start" : "1B",
        "end" : null,
        "outBase" : "2B",
        "isOut" : true,
        "outNumber" : 3
      },
      "details" : {
        "event" : "Forceout",
        "eventType" : "force_out",
        "movementReason" : "r_runner_out",
        "runner" : {
          "id" : 542458,
          "fullName" : "Wader Perez",
          "link" : "/api/v1/people/542458"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 542477,
          "link" : "/api/v1/people/542477"
        },
        "position" : {
          "code" : "4",
          "name" : "Second Base",
          "type" : "Infielder",
          "abbreviation" : "2B"
        },
        "credit" : "f_assist"
      }, {
        "player" : {
          "id" : 542340,
          "link" : "/api/v1/people/542340"
        },
        "position" : {
          "code" : "6",
          "name" : "Shortstop",
          "type" : "Infielder",
          "abbreviation" : "SS"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 101.29,
          "y" : 126.06
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "ground_ball",
        "hardness" : "medium",
        "location" : "4",
        "coordinates" : {
          "coordX" : 138.55,
          "coordY" : 166.67
        }
      },
      "index" : 0,
      "playId" : "02401986-0236-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 22
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Strikeout",
      "eventType" : "strikeout",
      "description" : "Rudney Balentien strikes out swinging.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 23,
      "halfInning" : "bottom",
      "inning" : 3,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 14
    },
    "count" : {
      "balls" : 0,
      "strikes" : 3,
      "outs" : 1
    },
    "matchup" : {
      "batter" : {
        "id" : 516680,
        "fullName" : "Rudney Balentien",
        "link" : "/api/v1/people/516680"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542203,
        "fullName" : "Angel De Jesus",
        "link" : "/api/v1/people/542203"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0, 1, 2 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 1
      },
      "details" : {
        "event" : "Strikeout",
        "eventType" : "strikeout",
        "movementReason" : null,
        "runner" : {
          "id" : 516680,
          "fullName" : "Rudney Balentien",
          "link" : "/api/v1/people/516680"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 2
      },
      "credits" : [ {
        "player" : {
          "id" : 542233,
          "link" : "/api/v1/people/542233"
        },
        "position" : {
          "code" : "2",
          "name" : "Catcher",
          "type" : "Catcher",
          "abbreviation" : "C"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 1
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 91.85,
          "y" : 124.34
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0246-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 2
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 95.28,
          "y" : 124.34
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0246-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 3
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 95.28,
          "y" : 121.75
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0246-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 23
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Flyout",
      "eventType" : "field_out",
      "description" : "Pedro Aguilar flies out to left fielder Santo Sandoval.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 24,
      "halfInning" : "bottom",
      "inning" : 3,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 516729,
        "fullName" : "Pedro Aguilar",
        "link" : "/api/v1/people/516729"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542203,
        "fullName" : "Angel De Jesus",
        "link" : "/api/v1/people/542203"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 2
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : null,
        "runner" : {
          "id" : 516729,
          "fullName" : "Pedro Aguilar",
          "link" : "/api/v1/people/516729"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 542468,
          "link" : "/api/v1/people/542468"
        },
        "position" : {
          "code" : "7",
          "name" : "Outfielder",
          "type" : "Outfielder",
          "abbreviation" : "LF"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 89.27,
          "y" : 126.93
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "fly_ball",
        "hardness" : "medium",
        "location" : "7",
        "coordinates" : {
          "coordX" : 67.27,
          "coordY" : 111.45
        }
      },
      "index" : 0,
      "playId" : "02401986-0256-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 24
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Single",
      "eventType" : "single",
      "description" : "Geancarlo Mendez singles on a line drive to left fielder Santo Sandoval.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 25,
      "halfInning" : "bottom",
      "inning" : 3,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 33
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 542479,
        "fullName" : "Geancarlo Mendez",
        "link" : "/api/v1/people/542479"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542203,
        "fullName" : "Angel De Jesus",
        "link" : "/api/v1/people/542203"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Men_On"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : null,
        "runner" : {
          "id" : 542479,
          "fullName" : "Geancarlo Mendez",
          "link" : "/api/v1/people/542479"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 542468,
          "link" : "/api/v1/people/542468"
        },
        "position" : {
          "code" : "7",
          "name" : "Outfielder",
          "type" : "Outfielder",
          "abbreviation" : "LF"
        },
        "credit" : "f_fielded_ball"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "D",
          "description" : "Hit Into Play - No Out(s)"
        },
        "description" : "In play, no out",
        "code" : "D",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 81.55,
          "y" : 113.11
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "line_drive",
        "hardness" : "medium",
        "location" : "7",
        "coordinates" : {
          "coordX" : 83.33,
          "coordY" : 131.53
        }
      },
      "index" : 0,
      "playId" : "02401986-0266-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 25
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Strikeout",
      "eventType" : "strikeout",
      "description" : "Emmanuel Checo strikes out swinging.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 26,
      "halfInning" : "bottom",
      "inning" : 3,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 14
    },
    "count" : {
      "balls" : 0,
      "strikes" : 3,
      "outs" : 3
    },
    "matchup" : {
      "batter" : {
        "id" : 501517,
        "fullName" : "Emmanuel Checo",
        "link" : "/api/v1/people/501517"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542203,
        "fullName" : "Angel De Jesus",
        "link" : "/api/v1/people/542203"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0, 1, 2 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 3
      },
      "details" : {
        "event" : "Strikeout",
        "eventType" : "strikeout",
        "movementReason" : null,
        "runner" : {
          "id" : 501517,
          "fullName" : "Emmanuel Checo",
          "link" : "/api/v1/people/501517"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 2
      },
      "credits" : [ {
        "player" : {
          "id" : 542233,
          "link" : "/api/v1/people/542233"
        },
        "position" : {
          "code" : "2",
          "name" : "Catcher",
          "type" : "Catcher",
          "abbreviation" : "C"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 1
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 94.42,
          "y" : 126.06
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0276-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 2
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 98.71,
          "y" : 116.57
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0276-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 3
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 99.57,
          "y" : 132.97
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0276-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 26
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Groundout",
      "eventType" : "field_out",
      "description" : "Audris Perez grounds out, third baseman Luis Paulino to first baseman Geancarlo Mendez.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 27,
      "halfInning" : "top",
      "inning" : 4,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 1
    },
    "matchup" : {
      "batter" : {
        "id" : 542233,
        "fullName" : "Audry Perez",
        "link" : "/api/v1/people/542233"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 516710,
        "fullName" : "Siulman Lebron",
        "link" : "/api/v1/people/516710"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 1
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : null,
        "runner" : {
          "id" : 542233,
          "fullName" : "Audry Perez",
          "link" : "/api/v1/people/542233"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 516760,
          "link" : "/api/v1/people/516760"
        },
        "position" : {
          "code" : "5",
          "name" : "Third Base",
          "type" : "Infielder",
          "abbreviation" : "3B"
        },
        "credit" : "f_assist"
      }, {
        "player" : {
          "id" : 542479,
          "link" : "/api/v1/people/542479"
        },
        "position" : {
          "code" : "3",
          "name" : "First Base",
          "type" : "Infielder",
          "abbreviation" : "1B"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 91.85,
          "y" : 124.34
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "ground_ball",
        "hardness" : "medium",
        "location" : "5",
        "coordinates" : {
          "coordX" : 103.41,
          "coordY" : 176.71
        }
      },
      "index" : 0,
      "playId" : "02401986-0286-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 27
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Strikeout",
      "eventType" : "strikeout",
      "description" : "Luis Pimentel strikes out swinging.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 28,
      "halfInning" : "top",
      "inning" : 4,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 14
    },
    "count" : {
      "balls" : 0,
      "strikes" : 3,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 540938,
        "fullName" : "Luis Pimentel",
        "link" : "/api/v1/people/540938"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 516710,
        "fullName" : "Siulman Lebron",
        "link" : "/api/v1/people/516710"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0, 1, 2 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 2
      },
      "details" : {
        "event" : "Strikeout",
        "eventType" : "strikeout",
        "movementReason" : null,
        "runner" : {
          "id" : 540938,
          "fullName" : "Luis Pimentel",
          "link" : "/api/v1/people/540938"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 2
      },
      "credits" : [ {
        "player" : {
          "id" : 501517,
          "link" : "/api/v1/people/501517"
        },
        "position" : {
          "code" : "2",
          "name" : "Catcher",
          "type" : "Catcher",
          "abbreviation" : "C"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 1
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 83.26,
          "y" : 125.2
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0296-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 2
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 97.0,
          "y" : 119.16
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0296-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 3
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 96.14,
          "y" : 116.57
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0296-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 28
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Pop Out",
      "eventType" : "field_out",
      "description" : "Santo Sandoval pops out to first baseman Geancarlo Mendez in foul territory.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 29,
      "halfInning" : "top",
      "inning" : 4,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 3
    },
    "matchup" : {
      "batter" : {
        "id" : 542468,
        "fullName" : "Santo Sandoval",
        "link" : "/api/v1/people/542468"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 516710,
        "fullName" : "Siulman Lebron",
        "link" : "/api/v1/people/516710"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 3
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : null,
        "runner" : {
          "id" : 542468,
          "fullName" : "Santo Sandoval",
          "link" : "/api/v1/people/542468"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 542479,
          "link" : "/api/v1/people/542479"
        },
        "position" : {
          "code" : "3",
          "name" : "First Base",
          "type" : "Infielder",
          "abbreviation" : "1B"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 91.85,
          "y" : 116.57
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "popup",
        "hardness" : "medium",
        "location" : "3",
        "coordinates" : {
          "coordX" : 153.61,
          "coordY" : 191.77
        }
      },
      "index" : 0,
      "playId" : "02401986-0306-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 29
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Pop Out",
      "eventType" : "field_out",
      "description" : "Luis Paulino pops out to shortstop Juan B Cabrera.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 30,
      "halfInning" : "bottom",
      "inning" : 4,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 1
    },
    "matchup" : {
      "batter" : {
        "id" : 516760,
        "fullName" : "Luis Paulino",
        "link" : "/api/v1/people/516760"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542203,
        "fullName" : "Angel De Jesus",
        "link" : "/api/v1/people/542203"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 1
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : null,
        "runner" : {
          "id" : 516760,
          "fullName" : "Luis Paulino",
          "link" : "/api/v1/people/516760"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 516868,
          "link" : "/api/v1/people/516868"
        },
        "position" : {
          "code" : "6",
          "name" : "Shortstop",
          "type" : "Infielder",
          "abbreviation" : "SS"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 84.98,
          "y" : 120.88
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "popup",
        "hardness" : "medium",
        "location" : "6",
        "coordinates" : {
          "coordX" : 99.4,
          "coordY" : 146.59
        }
      },
      "index" : 0,
      "playId" : "02401986-0316-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 30
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Groundout",
      "eventType" : "field_out",
      "description" : "Jose Trinidad grounds out, shortstop Juan B Cabrera to first baseman Marcos Martinez.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 31,
      "halfInning" : "bottom",
      "inning" : 4,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 542339,
        "fullName" : "Jose Trinidad",
        "link" : "/api/v1/people/542339"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542203,
        "fullName" : "Angel De Jesus",
        "link" : "/api/v1/people/542203"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 2
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : null,
        "runner" : {
          "id" : 542339,
          "fullName" : "Jose Trinidad",
          "link" : "/api/v1/people/542339"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 516868,
          "link" : "/api/v1/people/516868"
        },
        "position" : {
          "code" : "6",
          "name" : "Shortstop",
          "type" : "Infielder",
          "abbreviation" : "SS"
        },
        "credit" : "f_assist"
      }, {
        "player" : {
          "id" : 516658,
          "link" : "/api/v1/people/516658"
        },
        "position" : {
          "code" : "3",
          "name" : "First Base",
          "type" : "Infielder",
          "abbreviation" : "1B"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 92.7,
          "y" : 116.57
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "ground_ball",
        "hardness" : "medium",
        "location" : "6",
        "coordinates" : {
          "coordX" : 106.43,
          "coordY" : 169.68
        }
      },
      "index" : 0,
      "playId" : "02401986-0326-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 31
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Walk",
      "eventType" : "walk",
      "description" : "Carlos Best walks.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 32,
      "halfInning" : "bottom",
      "inning" : 4,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 4,
      "strikes" : 0,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 542758,
        "fullName" : "Carlos Best",
        "link" : "/api/v1/people/542758"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542203,
        "fullName" : "Angel De Jesus",
        "link" : "/api/v1/people/542203"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Men_On"
      }
    },
    "pitchIndex" : [ 0, 1, 2, 3 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Walk",
        "eventType" : "walk",
        "movementReason" : null,
        "runner" : {
          "id" : 542758,
          "fullName" : "Carlos Best",
          "link" : "/api/v1/people/542758"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 3
      },
      "credits" : [ ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 52.36,
          "y" : 107.93
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0336-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 2,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 56.65,
          "y" : 112.25
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0336-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 3,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 48.07,
          "y" : 110.52
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0336-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 4,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 63.52,
          "y" : 123.47
        },
        "breaks" : { }
      },
      "index" : 3,
      "playId" : "02401986-0336-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 32
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Groundout",
      "eventType" : "field_out",
      "description" : "Jonathan Villan grounds out, second baseman Wader Perez to first baseman Marcos Martinez.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 33,
      "halfInning" : "bottom",
      "inning" : 4,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 3
    },
    "matchup" : {
      "batter" : {
        "id" : 542340,
        "fullName" : "Jonathan Villar",
        "link" : "/api/v1/people/542340"
      },
      "batSide" : {
        "code" : "L",
        "description" : "Left"
      },
      "pitcher" : {
        "id" : 542203,
        "fullName" : "Angel De Jesus",
        "link" : "/api/v1/people/542203"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_LHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0, 2 ],
    "actionIndex" : [ 1 ],
    "runnerIndex" : [ 1 ],
    "runners" : [ {
      "movement" : {
        "start" : "1B",
        "end" : "2B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Pickoff Error 1B",
        "eventType" : "pickoff_error_1b",
        "movementReason" : "r_adv_play",
        "runner" : {
          "id" : 542758,
          "fullName" : "Carlos Best",
          "link" : "/api/v1/people/542758"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 1
      },
      "credits" : [ {
        "player" : {
          "id" : 542203,
          "link" : "/api/v1/people/542203"
        },
        "position" : {
          "code" : "1",
          "name" : "Pitcher",
          "type" : "Pitcher",
          "abbreviation" : "P"
        },
        "credit" : "f_throwing_error"
      } ]
    }, {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 3
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : null,
        "runner" : {
          "id" : 542340,
          "fullName" : "Jonathan Villar",
          "link" : "/api/v1/people/542340"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 2
      },
      "credits" : [ {
        "player" : {
          "id" : 542458,
          "link" : "/api/v1/people/542458"
        },
        "position" : {
          "code" : "4",
          "name" : "Second Base",
          "type" : "Infielder",
          "abbreviation" : "2B"
        },
        "credit" : "f_assist"
      }, {
        "player" : {
          "id" : 516658,
          "link" : "/api/v1/people/516658"
        },
        "position" : {
          "code" : "3",
          "name" : "First Base",
          "type" : "Infielder",
          "abbreviation" : "1B"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "description" : "Pickoff Attempt 1B",
        "code" : "1",
        "hasReview" : false,
        "fromCatcher" : false
      },
      "count" : { },
      "index" : 0,
      "playId" : "02401986-0346-0003-001c-f08cd117d70a",
      "isPitch" : false,
      "type" : "pickoff"
    }, {
      "details" : {
        "description" : "With Jonathan Villan batting, throwing error by Angel De Jesus on the pickoff attempt, Carlos Best to 2nd.",
        "event" : "Pickoff Error 1B",
        "eventType" : "pickoff_error_1b",
        "awayScore" : 1,
        "homeScore" : 4,
        "isScoringPlay" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0,
        "outs" : 2
      },
      "index" : 1,
      "isPitch" : false,
      "type" : "action",
      "player" : {
        "id" : 542203,
        "link" : "/api/v1/people/542203"
      }
    }, {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 94.42,
          "y" : 131.24
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "ground_ball",
        "hardness" : "medium",
        "location" : "4",
        "coordinates" : {
          "coordX" : 134.54,
          "coordY" : 163.65
        }
      },
      "index" : 2,
      "playId" : "02401986-0346-0003-001c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 33
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Pop Out",
      "eventType" : "field_out",
      "description" : "Bernardo Villar pops out to second baseman Carlos Valenzuela.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 34,
      "halfInning" : "top",
      "inning" : 5,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 1
    },
    "matchup" : {
      "batter" : {
        "id" : 542248,
        "fullName" : "Bernardo Villar",
        "link" : "/api/v1/people/542248"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 516710,
        "fullName" : "Siulman Lebron",
        "link" : "/api/v1/people/516710"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 1
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : null,
        "runner" : {
          "id" : 542248,
          "fullName" : "Bernardo Villar",
          "link" : "/api/v1/people/542248"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 542477,
          "link" : "/api/v1/people/542477"
        },
        "position" : {
          "code" : "4",
          "name" : "Second Base",
          "type" : "Infielder",
          "abbreviation" : "2B"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 97.0,
          "y" : 120.88
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "popup",
        "hardness" : "medium",
        "location" : "4",
        "coordinates" : {
          "coordX" : 135.54,
          "coordY" : 136.55
        }
      },
      "index" : 0,
      "playId" : "02401986-0356-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 34
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Groundout",
      "eventType" : "field_out",
      "description" : "Roberto Reyes grounds out, shortstop Jonathan Villan to first baseman Geancarlo Mendez.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 35,
      "halfInning" : "top",
      "inning" : 5,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 542465,
        "fullName" : "Roberto Reyes",
        "link" : "/api/v1/people/542465"
      },
      "batSide" : {
        "code" : "L",
        "description" : "Left"
      },
      "pitcher" : {
        "id" : 516710,
        "fullName" : "Siulman Lebron",
        "link" : "/api/v1/people/516710"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_LHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 2
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : null,
        "runner" : {
          "id" : 542465,
          "fullName" : "Roberto Reyes",
          "link" : "/api/v1/people/542465"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 542340,
          "link" : "/api/v1/people/542340"
        },
        "position" : {
          "code" : "6",
          "name" : "Shortstop",
          "type" : "Infielder",
          "abbreviation" : "SS"
        },
        "credit" : "f_assist"
      }, {
        "player" : {
          "id" : 542479,
          "link" : "/api/v1/people/542479"
        },
        "position" : {
          "code" : "3",
          "name" : "First Base",
          "type" : "Infielder",
          "abbreviation" : "1B"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 87.55,
          "y" : 126.06
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "ground_ball",
        "hardness" : "medium",
        "location" : "6",
        "coordinates" : {
          "coordX" : 108.43,
          "coordY" : 167.67
        }
      },
      "index" : 0,
      "playId" : "02401986-0366-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 35
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Strikeout",
      "eventType" : "strikeout",
      "description" : "Marcos Martinez strikes out swinging.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 36,
      "halfInning" : "top",
      "inning" : 5,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 14
    },
    "count" : {
      "balls" : 0,
      "strikes" : 3,
      "outs" : 3
    },
    "matchup" : {
      "batter" : {
        "id" : 516658,
        "fullName" : "Marcos Martinez",
        "link" : "/api/v1/people/516658"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 516710,
        "fullName" : "Siulman Lebron",
        "link" : "/api/v1/people/516710"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0, 1, 2 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 3
      },
      "details" : {
        "event" : "Strikeout",
        "eventType" : "strikeout",
        "movementReason" : null,
        "runner" : {
          "id" : 516658,
          "fullName" : "Marcos Martinez",
          "link" : "/api/v1/people/516658"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 2
      },
      "credits" : [ {
        "player" : {
          "id" : 501517,
          "link" : "/api/v1/people/501517"
        },
        "position" : {
          "code" : "2",
          "name" : "Catcher",
          "type" : "Catcher",
          "abbreviation" : "C"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 1
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 95.28,
          "y" : 130.38
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0376-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 2
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 92.7,
          "y" : 121.75
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0376-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 3
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 93.56,
          "y" : 120.88
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0376-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 36
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Strikeout",
      "eventType" : "strikeout",
      "description" : "Carlos Valenzuela strikes out swinging.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 37,
      "halfInning" : "bottom",
      "inning" : 5,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 14
    },
    "count" : {
      "balls" : 0,
      "strikes" : 3,
      "outs" : 1
    },
    "matchup" : {
      "batter" : {
        "id" : 542477,
        "fullName" : "Carlos Valenzuela",
        "link" : "/api/v1/people/542477"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542241,
        "fullName" : "Eddy Rivera",
        "link" : "/api/v1/people/542241"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 1, 2, 3 ],
    "actionIndex" : [ 0 ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 1
      },
      "details" : {
        "event" : "Strikeout",
        "eventType" : "strikeout",
        "movementReason" : null,
        "runner" : {
          "id" : 542477,
          "fullName" : "Carlos Valenzuela",
          "link" : "/api/v1/people/542477"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 3
      },
      "credits" : [ {
        "player" : {
          "id" : 542233,
          "link" : "/api/v1/people/542233"
        },
        "position" : {
          "code" : "2",
          "name" : "Catcher",
          "type" : "Catcher",
          "abbreviation" : "C"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "description" : "Pitcher Change: Eddy Rivera replaces Angel De Jesus.",
        "event" : "Pitching Substitution",
        "awayScore" : 1,
        "homeScore" : 4,
        "isScoringPlay" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0,
        "outs" : 0
      },
      "index" : 0,
      "isPitch" : false,
      "type" : "action",
      "player" : {
        "id" : 542241,
        "link" : "/api/v1/people/542241"
      },
      "position" : {
        "code" : "1",
        "name" : "Pitcher",
        "type" : "Pitcher",
        "abbreviation" : "P"
      }
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 1
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 92.7,
          "y" : 126.93
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0386-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 2
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 89.27,
          "y" : 118.29
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0386-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 3
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 84.12,
          "y" : 121.75
        },
        "breaks" : { }
      },
      "index" : 3,
      "playId" : "02401986-0386-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 37
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Field Error",
      "eventType" : "field_error",
      "description" : "Rudney Balentien reaches on fielding error by third baseman Luis Pimentel.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 38,
      "halfInning" : "bottom",
      "inning" : 5,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 1
    },
    "matchup" : {
      "batter" : {
        "id" : 516680,
        "fullName" : "Rudney Balentien",
        "link" : "/api/v1/people/516680"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542241,
        "fullName" : "Eddy Rivera",
        "link" : "/api/v1/people/542241"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Men_On"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Field Error",
        "eventType" : "field_error",
        "movementReason" : "r_adv_play",
        "runner" : {
          "id" : 516680,
          "fullName" : "Rudney Balentien",
          "link" : "/api/v1/people/516680"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 540938,
          "link" : "/api/v1/people/540938"
        },
        "position" : {
          "code" : "5",
          "name" : "Third Base",
          "type" : "Infielder",
          "abbreviation" : "3B"
        },
        "credit" : "f_fielding_error"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "D",
          "description" : "Hit Into Play - No Out(s)"
        },
        "description" : "In play, no out",
        "code" : "D",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 100.43,
          "y" : 126.93
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "ground_ball",
        "hardness" : "medium",
        "location" : "5",
        "coordinates" : {
          "coordX" : 97.39,
          "coordY" : 174.7
        }
      },
      "index" : 0,
      "playId" : "02401986-0396-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 38
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Strikeout",
      "eventType" : "strikeout",
      "description" : "Pedro Aguilar strikes out swinging.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 39,
      "halfInning" : "bottom",
      "inning" : 5,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 14
    },
    "count" : {
      "balls" : 0,
      "strikes" : 3,
      "outs" : 3
    },
    "matchup" : {
      "batter" : {
        "id" : 516729,
        "fullName" : "Pedro Aguilar",
        "link" : "/api/v1/people/516729"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542241,
        "fullName" : "Eddy Rivera",
        "link" : "/api/v1/people/542241"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0, 2, 3 ],
    "actionIndex" : [ 1 ],
    "runnerIndex" : [ 1 ],
    "runners" : [ {
      "movement" : {
        "start" : "1B",
        "end" : null,
        "outBase" : "2B",
        "isOut" : true,
        "outNumber" : 2
      },
      "details" : {
        "event" : "Caught Stealing 2B",
        "eventType" : "caught_stealing_2b",
        "movementReason" : "r_caught_stealing_2b",
        "runner" : {
          "id" : 516680,
          "fullName" : "Rudney Balentien",
          "link" : "/api/v1/people/516680"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 1
      },
      "credits" : [ {
        "player" : {
          "id" : 542233,
          "link" : "/api/v1/people/542233"
        },
        "position" : {
          "code" : "2",
          "name" : "Catcher",
          "type" : "Catcher",
          "abbreviation" : "C"
        },
        "credit" : "f_assist"
      }, {
        "player" : {
          "id" : 516868,
          "link" : "/api/v1/people/516868"
        },
        "position" : {
          "code" : "6",
          "name" : "Shortstop",
          "type" : "Infielder",
          "abbreviation" : "SS"
        },
        "credit" : "f_putout"
      } ]
    }, {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 3
      },
      "details" : {
        "event" : "Strikeout",
        "eventType" : "strikeout",
        "movementReason" : null,
        "runner" : {
          "id" : 516729,
          "fullName" : "Pedro Aguilar",
          "link" : "/api/v1/people/516729"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 3
      },
      "credits" : [ {
        "player" : {
          "id" : 542233,
          "link" : "/api/v1/people/542233"
        },
        "position" : {
          "code" : "2",
          "name" : "Catcher",
          "type" : "Catcher",
          "abbreviation" : "C"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "C",
          "description" : "Strike - Called"
        },
        "description" : "Called Strike",
        "code" : "C",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 1
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 90.13,
          "y" : 126.06
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0406-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "description" : "With Pedro Aguilar batting, Rudney Balentien caught stealing 2nd base, catcher Audris Perez to shortstop Juan B Cabrera.",
        "event" : "Caught Stealing 2B",
        "eventType" : "caught_stealing_2b",
        "awayScore" : 1,
        "homeScore" : 4,
        "isScoringPlay" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 1,
        "outs" : 2
      },
      "index" : 1,
      "isPitch" : false,
      "type" : "action",
      "player" : {
        "id" : 516680,
        "link" : "/api/v1/people/516680"
      }
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 2
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 98.71,
          "y" : 125.2
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0406-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 3
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 90.99,
          "y" : 126.93
        },
        "breaks" : { }
      },
      "index" : 3,
      "playId" : "02401986-0406-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 39
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Groundout",
      "eventType" : "field_out",
      "description" : "Wader Perez grounds out, shortstop Jonathan Villan to first baseman Geancarlo Mendez.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 40,
      "halfInning" : "top",
      "inning" : 6,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 1
    },
    "matchup" : {
      "batter" : {
        "id" : 542458,
        "fullName" : "Wader Perez",
        "link" : "/api/v1/people/542458"
      },
      "batSide" : {
        "code" : "L",
        "description" : "Left"
      },
      "pitcher" : {
        "id" : 516710,
        "fullName" : "Siulman Lebron",
        "link" : "/api/v1/people/516710"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_LHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 1 ],
    "actionIndex" : [ 0 ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 1
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : null,
        "runner" : {
          "id" : 542458,
          "fullName" : "Wader Perez",
          "link" : "/api/v1/people/542458"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 1
      },
      "credits" : [ {
        "player" : {
          "id" : 542340,
          "link" : "/api/v1/people/542340"
        },
        "position" : {
          "code" : "6",
          "name" : "Shortstop",
          "type" : "Infielder",
          "abbreviation" : "SS"
        },
        "credit" : "f_assist"
      }, {
        "player" : {
          "id" : 542479,
          "link" : "/api/v1/people/542479"
        },
        "position" : {
          "code" : "3",
          "name" : "First Base",
          "type" : "Infielder",
          "abbreviation" : "1B"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "description" : "Defensive Substitution:  Miguel Alvarez replaces center fielder Rudney Balentien, batting 3rd, playing center field.",
        "event" : "Defensive Sub",
        "eventType" : "defensive_substitution",
        "awayScore" : 1,
        "homeScore" : 4,
        "isScoringPlay" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0,
        "outs" : 0
      },
      "index" : 0,
      "isPitch" : false,
      "type" : "action",
      "player" : {
        "id" : 516758,
        "link" : "/api/v1/people/516758"
      },
      "battingOrder" : "301"
    }, {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 100.43,
          "y" : 114.84
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "ground_ball",
        "hardness" : "medium",
        "location" : "6",
        "coordinates" : {
          "coordX" : 108.43,
          "coordY" : 165.66
        }
      },
      "index" : 1,
      "playId" : "02401986-0416-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 40
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Triple",
      "eventType" : "triple",
      "description" : "Alexander Castellano triples (3) on a line drive to center fielder Miguel Alvarez.",
      "rbi" : 0,
      "awayScore" : 1,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 41,
      "halfInning" : "top",
      "inning" : 6,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 35
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 1
    },
    "matchup" : {
      "batter" : {
        "id" : 542463,
        "fullName" : "Alex Castellano",
        "link" : "/api/v1/people/542463"
      },
      "batSide" : {
        "code" : "L",
        "description" : "Left"
      },
      "pitcher" : {
        "id" : 516710,
        "fullName" : "Siulman Lebron",
        "link" : "/api/v1/people/516710"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_LHB",
        "menOnBase" : "RISP"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : "3B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Triple",
        "eventType" : "triple",
        "movementReason" : null,
        "runner" : {
          "id" : 542463,
          "fullName" : "Alex Castellano",
          "link" : "/api/v1/people/542463"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 516758,
          "link" : "/api/v1/people/516758"
        },
        "position" : {
          "code" : "8",
          "name" : "Outfielder",
          "type" : "Outfielder",
          "abbreviation" : "CF"
        },
        "credit" : "f_fielded_ball"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "D",
          "description" : "Hit Into Play - No Out(s)"
        },
        "description" : "In play, no out",
        "code" : "D",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 100.43,
          "y" : 118.29
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "line_drive",
        "hardness" : "medium",
        "location" : "8",
        "coordinates" : {
          "coordX" : 125.5,
          "coordY" : 57.23
        }
      },
      "index" : 0,
      "playId" : "02401986-0426-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 41
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Groundout",
      "eventType" : "field_out",
      "description" : "Juan B Cabrera grounds out, shortstop Jonathan Villan to first baseman Geancarlo Mendez.    Alexander Castellano scores.",
      "rbi" : 1,
      "awayScore" : 2,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 42,
      "halfInning" : "top",
      "inning" : 6,
      "isComplete" : true,
      "isScoringPlay" : true,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 516868,
        "fullName" : "Juan B Cabrera",
        "link" : "/api/v1/people/516868"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 516710,
        "fullName" : "Siulman Lebron",
        "link" : "/api/v1/people/516710"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0, 1 ],
    "runners" : [ {
      "movement" : {
        "start" : "3B",
        "end" : "score",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : "r_adv_play",
        "runner" : {
          "id" : 542463,
          "fullName" : "Alex Castellano",
          "link" : "/api/v1/people/542463"
        },
        "responsiblePitcher" : {
          "id" : 516710,
          "link" : "/api/v1/people/516710"
        },
        "isScoringEvent" : true,
        "rbi" : true,
        "earned" : true,
        "teamUnearned" : false,
        "playIndex" : 0
      }
    }, {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 2
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : null,
        "runner" : {
          "id" : 516868,
          "fullName" : "Juan B Cabrera",
          "link" : "/api/v1/people/516868"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 542340,
          "link" : "/api/v1/people/542340"
        },
        "position" : {
          "code" : "6",
          "name" : "Shortstop",
          "type" : "Infielder",
          "abbreviation" : "SS"
        },
        "credit" : "f_assist"
      }, {
        "player" : {
          "id" : 542479,
          "link" : "/api/v1/people/542479"
        },
        "position" : {
          "code" : "3",
          "name" : "First Base",
          "type" : "Infielder",
          "abbreviation" : "1B"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "E",
          "description" : "Hit Into Play - Run(s)"
        },
        "description" : "In play, run(s)",
        "code" : "E",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 85.84,
          "y" : 124.34
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "ground_ball",
        "hardness" : "medium",
        "location" : "6",
        "coordinates" : {
          "coordX" : 106.43,
          "coordY" : 164.66
        }
      },
      "index" : 0,
      "playId" : "02401986-0436-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 42
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Flyout",
      "eventType" : "field_out",
      "description" : "Audris Perez flies out to center fielder Miguel Alvarez.",
      "rbi" : 0,
      "awayScore" : 2,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 43,
      "halfInning" : "top",
      "inning" : 6,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 3
    },
    "matchup" : {
      "batter" : {
        "id" : 542233,
        "fullName" : "Audry Perez",
        "link" : "/api/v1/people/542233"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 516710,
        "fullName" : "Siulman Lebron",
        "link" : "/api/v1/people/516710"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 3
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : null,
        "runner" : {
          "id" : 542233,
          "fullName" : "Audry Perez",
          "link" : "/api/v1/people/542233"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 516758,
          "link" : "/api/v1/people/516758"
        },
        "position" : {
          "code" : "8",
          "name" : "Outfielder",
          "type" : "Outfielder",
          "abbreviation" : "CF"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 93.56,
          "y" : 127.79
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "fly_ball",
        "hardness" : "medium",
        "location" : "8",
        "coordinates" : {
          "coordX" : 112.45,
          "coordY" : 111.45
        }
      },
      "index" : 0,
      "playId" : "02401986-0446-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 43
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Flyout",
      "eventType" : "field_out",
      "description" : "Geancarlo Mendez flies out to left fielder Santo Sandoval.",
      "rbi" : 0,
      "awayScore" : 2,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 44,
      "halfInning" : "bottom",
      "inning" : 6,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 1
    },
    "matchup" : {
      "batter" : {
        "id" : 542479,
        "fullName" : "Geancarlo Mendez",
        "link" : "/api/v1/people/542479"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542241,
        "fullName" : "Eddy Rivera",
        "link" : "/api/v1/people/542241"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 1
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : null,
        "runner" : {
          "id" : 542479,
          "fullName" : "Geancarlo Mendez",
          "link" : "/api/v1/people/542479"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 542468,
          "link" : "/api/v1/people/542468"
        },
        "position" : {
          "code" : "7",
          "name" : "Outfielder",
          "type" : "Outfielder",
          "abbreviation" : "LF"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 82.4,
          "y" : 121.75
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "fly_ball",
        "hardness" : "medium",
        "location" : "7",
        "coordinates" : {
          "coordX" : 57.23,
          "coordY" : 116.47
        }
      },
      "index" : 0,
      "playId" : "02401986-0456-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 44
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Hit By Pitch",
      "eventType" : "hit_by_pitch",
      "description" : "Emmanuel Checo hit by pitch.",
      "rbi" : 0,
      "awayScore" : 2,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 45,
      "halfInning" : "bottom",
      "inning" : 6,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 11
    },
    "count" : {
      "balls" : 1,
      "strikes" : 0,
      "outs" : 1
    },
    "matchup" : {
      "batter" : {
        "id" : 501517,
        "fullName" : "Emmanuel Checo",
        "link" : "/api/v1/people/501517"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542241,
        "fullName" : "Eddy Rivera",
        "link" : "/api/v1/people/542241"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Men_On"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Hit By Pitch",
        "eventType" : "hit_by_pitch",
        "movementReason" : null,
        "runner" : {
          "id" : 501517,
          "fullName" : "Emmanuel Checo",
          "link" : "/api/v1/people/501517"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "H",
          "description" : "Ball - Hit by Pitch"
        },
        "description" : "Hit By Pitch",
        "code" : "H",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 98.71,
          "y" : 129.52
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0466-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 45
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Groundout",
      "eventType" : "field_out",
      "description" : "Luis Paulino grounds out, shortstop Juan B Cabrera to first baseman Marcos Martinez.    Emmanuel Checo to 3rd.",
      "rbi" : 0,
      "awayScore" : 2,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 46,
      "halfInning" : "bottom",
      "inning" : 6,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 1,
      "strikes" : 0,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 516760,
        "fullName" : "Luis Paulino",
        "link" : "/api/v1/people/516760"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542241,
        "fullName" : "Eddy Rivera",
        "link" : "/api/v1/people/542241"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "RISP"
      }
    },
    "pitchIndex" : [ 0, 2 ],
    "actionIndex" : [ 1 ],
    "runnerIndex" : [ 1, 2 ],
    "runners" : [ {
      "movement" : {
        "start" : "1B",
        "end" : "2B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Wild Pitch",
        "eventType" : "wild_pitch",
        "movementReason" : "r_adv_play",
        "runner" : {
          "id" : 501517,
          "fullName" : "Emmanuel Checo",
          "link" : "/api/v1/people/501517"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 1
      },
      "credits" : [ ]
    }, {
      "movement" : {
        "start" : "2B",
        "end" : "3B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : "r_adv_play",
        "runner" : {
          "id" : 501517,
          "fullName" : "Emmanuel Checo",
          "link" : "/api/v1/people/501517"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 2
      }
    }, {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 2
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : null,
        "runner" : {
          "id" : 516760,
          "fullName" : "Luis Paulino",
          "link" : "/api/v1/people/516760"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 2
      },
      "credits" : [ {
        "player" : {
          "id" : 516868,
          "link" : "/api/v1/people/516868"
        },
        "position" : {
          "code" : "6",
          "name" : "Shortstop",
          "type" : "Infielder",
          "abbreviation" : "SS"
        },
        "credit" : "f_assist"
      }, {
        "player" : {
          "id" : 516658,
          "link" : "/api/v1/people/516658"
        },
        "position" : {
          "code" : "3",
          "name" : "First Base",
          "type" : "Infielder",
          "abbreviation" : "1B"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 50.64,
          "y" : 112.25
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0476-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "description" : "With Luis Paulino batting, wild pitch by Eddy Rivera, Emmanuel Checo to 2nd.",
        "event" : "Wild Pitch",
        "eventType" : "wild_pitch",
        "awayScore" : 2,
        "homeScore" : 4,
        "isScoringPlay" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0,
        "outs" : 1
      },
      "index" : 1,
      "isPitch" : false,
      "type" : "action",
      "player" : {
        "id" : 542233,
        "link" : "/api/v1/people/542233"
      }
    }, {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 97.0,
          "y" : 132.97
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "ground_ball",
        "hardness" : "medium",
        "location" : "6",
        "coordinates" : {
          "coordX" : 111.45,
          "coordY" : 165.66
        }
      },
      "index" : 2,
      "playId" : "02401986-0476-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 46
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Strikeout",
      "eventType" : "strikeout",
      "description" : "Jose Trinidad strikes out swinging.",
      "rbi" : 0,
      "awayScore" : 2,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 47,
      "halfInning" : "bottom",
      "inning" : 6,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 14
    },
    "count" : {
      "balls" : 0,
      "strikes" : 3,
      "outs" : 3
    },
    "matchup" : {
      "batter" : {
        "id" : 542339,
        "fullName" : "Jose Trinidad",
        "link" : "/api/v1/people/542339"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542241,
        "fullName" : "Eddy Rivera",
        "link" : "/api/v1/people/542241"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0, 1, 2 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 3
      },
      "details" : {
        "event" : "Strikeout",
        "eventType" : "strikeout",
        "movementReason" : null,
        "runner" : {
          "id" : 542339,
          "fullName" : "Jose Trinidad",
          "link" : "/api/v1/people/542339"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 2
      },
      "credits" : [ {
        "player" : {
          "id" : 542233,
          "link" : "/api/v1/people/542233"
        },
        "position" : {
          "code" : "2",
          "name" : "Catcher",
          "type" : "Catcher",
          "abbreviation" : "C"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 1
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 91.85,
          "y" : 125.2
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0486-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 2
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 84.98,
          "y" : 117.43
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0486-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 3
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 85.84,
          "y" : 113.98
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0486-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 47
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Flyout",
      "eventType" : "field_out",
      "description" : "Luis Pimentel flies out to center fielder Miguel Alvarez.",
      "rbi" : 0,
      "awayScore" : 2,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 48,
      "halfInning" : "top",
      "inning" : 7,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 1
    },
    "matchup" : {
      "batter" : {
        "id" : 540938,
        "fullName" : "Luis Pimentel",
        "link" : "/api/v1/people/540938"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 501651,
        "fullName" : "Raudy Sandoval",
        "link" : "/api/v1/people/501651"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 1 ],
    "actionIndex" : [ 0 ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 1
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : null,
        "runner" : {
          "id" : 540938,
          "fullName" : "Luis Pimentel",
          "link" : "/api/v1/people/540938"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 1
      },
      "credits" : [ {
        "player" : {
          "id" : 516758,
          "link" : "/api/v1/people/516758"
        },
        "position" : {
          "code" : "8",
          "name" : "Outfielder",
          "type" : "Outfielder",
          "abbreviation" : "CF"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "description" : "Pitcher Change: Raudy Sandoval replaces Siulman Lebron.",
        "event" : "Pitching Substitution",
        "awayScore" : 2,
        "homeScore" : 4,
        "isScoringPlay" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0,
        "outs" : 0
      },
      "index" : 0,
      "isPitch" : false,
      "type" : "action",
      "player" : {
        "id" : 501651,
        "link" : "/api/v1/people/501651"
      },
      "position" : {
        "code" : "1",
        "name" : "Pitcher",
        "type" : "Pitcher",
        "abbreviation" : "P"
      }
    }, {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 92.7,
          "y" : 121.75
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "fly_ball",
        "hardness" : "medium",
        "location" : "8",
        "coordinates" : {
          "coordX" : 125.5,
          "coordY" : 97.39
        }
      },
      "index" : 1,
      "playId" : "02401986-0496-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 48
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Hit By Pitch",
      "eventType" : "hit_by_pitch",
      "description" : "Santo Sandoval hit by pitch.",
      "rbi" : 0,
      "awayScore" : 2,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 49,
      "halfInning" : "top",
      "inning" : 7,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 11
    },
    "count" : {
      "balls" : 1,
      "strikes" : 0,
      "outs" : 1
    },
    "matchup" : {
      "batter" : {
        "id" : 542468,
        "fullName" : "Santo Sandoval",
        "link" : "/api/v1/people/542468"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 501651,
        "fullName" : "Raudy Sandoval",
        "link" : "/api/v1/people/501651"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Men_On"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Hit By Pitch",
        "eventType" : "hit_by_pitch",
        "movementReason" : null,
        "runner" : {
          "id" : 542468,
          "fullName" : "Santo Sandoval",
          "link" : "/api/v1/people/542468"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "H",
          "description" : "Ball - Hit by Pitch"
        },
        "description" : "Hit By Pitch",
        "code" : "H",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 95.28,
          "y" : 123.47
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0506-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 49
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Flyout",
      "eventType" : "field_out",
      "description" : "Bernardo Villar flies out to right fielder Carlos Best.",
      "rbi" : 0,
      "awayScore" : 2,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 50,
      "halfInning" : "top",
      "inning" : 7,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 1,
      "strikes" : 0,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 542248,
        "fullName" : "Bernardo Villar",
        "link" : "/api/v1/people/542248"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 501651,
        "fullName" : "Raudy Sandoval",
        "link" : "/api/v1/people/501651"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "RISP"
      }
    },
    "pitchIndex" : [ 0, 2 ],
    "actionIndex" : [ 1 ],
    "runnerIndex" : [ 1 ],
    "runners" : [ {
      "movement" : {
        "start" : "1B",
        "end" : "2B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Stolen Base 2B",
        "eventType" : "stolen_base_2b",
        "movementReason" : "r_stolen_base_2b",
        "runner" : {
          "id" : 542468,
          "fullName" : "Santo Sandoval",
          "link" : "/api/v1/people/542468"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 1
      },
      "credits" : [ ]
    }, {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 2
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : null,
        "runner" : {
          "id" : 542248,
          "fullName" : "Bernardo Villar",
          "link" : "/api/v1/people/542248"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 2
      },
      "credits" : [ {
        "player" : {
          "id" : 542758,
          "link" : "/api/v1/people/542758"
        },
        "position" : {
          "code" : "9",
          "name" : "Outfielder",
          "type" : "Outfielder",
          "abbreviation" : "RF"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 54.94,
          "y" : 110.52
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0516-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "description" : "With Bernardo Villar batting, Santo Sandoval steals (10) 2nd base.",
        "event" : "Stolen Base 2B",
        "eventType" : "stolen_base_2b",
        "awayScore" : 2,
        "homeScore" : 4,
        "isScoringPlay" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0,
        "outs" : 1
      },
      "index" : 1,
      "isPitch" : false,
      "type" : "action",
      "player" : {
        "id" : 542468,
        "link" : "/api/v1/people/542468"
      }
    }, {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 93.56,
          "y" : 131.24
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "fly_ball",
        "hardness" : "medium",
        "location" : "9",
        "coordinates" : {
          "coordX" : 164.66,
          "coordY" : 109.44
        }
      },
      "index" : 2,
      "playId" : "02401986-0516-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 50
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Hit By Pitch",
      "eventType" : "hit_by_pitch",
      "description" : "Roberto Reyes hit by pitch.",
      "rbi" : 0,
      "awayScore" : 2,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 51,
      "halfInning" : "top",
      "inning" : 7,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 11
    },
    "count" : {
      "balls" : 1,
      "strikes" : 0,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 542465,
        "fullName" : "Roberto Reyes",
        "link" : "/api/v1/people/542465"
      },
      "batSide" : {
        "code" : "L",
        "description" : "Left"
      },
      "pitcher" : {
        "id" : 501651,
        "fullName" : "Raudy Sandoval",
        "link" : "/api/v1/people/501651"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_LHB",
        "menOnBase" : "RISP"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Hit By Pitch",
        "eventType" : "hit_by_pitch",
        "movementReason" : null,
        "runner" : {
          "id" : 542465,
          "fullName" : "Roberto Reyes",
          "link" : "/api/v1/people/542465"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "H",
          "description" : "Ball - Hit by Pitch"
        },
        "description" : "Hit By Pitch",
        "code" : "H",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 84.12,
          "y" : 120.02
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0526-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 51
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Walk",
      "eventType" : "walk",
      "description" : "Marcos Martinez walks.    Santo Sandoval to 3rd.    Roberto Reyes to 2nd.",
      "rbi" : 0,
      "awayScore" : 2,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 52,
      "halfInning" : "top",
      "inning" : 7,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 4,
      "strikes" : 0,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 516658,
        "fullName" : "Marcos Martinez",
        "link" : "/api/v1/people/516658"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 501651,
        "fullName" : "Raudy Sandoval",
        "link" : "/api/v1/people/501651"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Loaded"
      }
    },
    "pitchIndex" : [ 0, 1, 2, 3 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0, 1, 2 ],
    "runners" : [ {
      "movement" : {
        "start" : "2B",
        "end" : "3B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Walk",
        "eventType" : "walk",
        "movementReason" : "r_adv_force",
        "runner" : {
          "id" : 542468,
          "fullName" : "Santo Sandoval",
          "link" : "/api/v1/people/542468"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 3
      }
    }, {
      "movement" : {
        "start" : "1B",
        "end" : "2B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Walk",
        "eventType" : "walk",
        "movementReason" : "r_adv_force",
        "runner" : {
          "id" : 542465,
          "fullName" : "Roberto Reyes",
          "link" : "/api/v1/people/542465"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 3
      }
    }, {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Walk",
        "eventType" : "walk",
        "movementReason" : null,
        "runner" : {
          "id" : 516658,
          "fullName" : "Marcos Martinez",
          "link" : "/api/v1/people/516658"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 3
      },
      "credits" : [ ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 50.64,
          "y" : 110.52
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0536-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 2,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 48.07,
          "y" : 106.2
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0536-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 3,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 51.5,
          "y" : 105.34
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0536-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 4,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 42.06,
          "y" : 98.43
        },
        "breaks" : { }
      },
      "index" : 3,
      "playId" : "02401986-0536-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 52
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Single",
      "eventType" : "single",
      "description" : "Wader Perez singles on a line drive to right fielder Carlos Best.    Santo Sandoval scores.    Roberto Reyes scores.    Marcos Martinez to 3rd.",
      "rbi" : 2,
      "awayScore" : 4,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 53,
      "halfInning" : "top",
      "inning" : 7,
      "isComplete" : true,
      "isScoringPlay" : true,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 70
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 542458,
        "fullName" : "Wader Perez",
        "link" : "/api/v1/people/542458"
      },
      "batSide" : {
        "code" : "L",
        "description" : "Left"
      },
      "pitcher" : {
        "id" : 525734,
        "fullName" : "Gabriel Arias",
        "link" : "/api/v1/people/525734"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_LHB",
        "menOnBase" : "RISP"
      }
    },
    "pitchIndex" : [ 1 ],
    "actionIndex" : [ 0 ],
    "runnerIndex" : [ 0, 1, 2, 3, 4, 5 ],
    "runners" : [ {
      "movement" : {
        "start" : "3B",
        "end" : "score",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : "r_adv_force",
        "runner" : {
          "id" : 542468,
          "fullName" : "Santo Sandoval",
          "link" : "/api/v1/people/542468"
        },
        "responsiblePitcher" : {
          "id" : 501651,
          "link" : "/api/v1/people/501651"
        },
        "isScoringEvent" : true,
        "rbi" : true,
        "earned" : true,
        "teamUnearned" : false,
        "playIndex" : 1
      }
    }, {
      "movement" : {
        "start" : "2B",
        "end" : "3B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : "r_adv_force",
        "runner" : {
          "id" : 542465,
          "fullName" : "Roberto Reyes",
          "link" : "/api/v1/people/542465"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 1
      }
    }, {
      "movement" : {
        "start" : "3B",
        "end" : "score",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : "r_adv_play",
        "runner" : {
          "id" : 542465,
          "fullName" : "Roberto Reyes",
          "link" : "/api/v1/people/542465"
        },
        "responsiblePitcher" : {
          "id" : 501651,
          "link" : "/api/v1/people/501651"
        },
        "isScoringEvent" : true,
        "rbi" : true,
        "earned" : true,
        "teamUnearned" : false,
        "playIndex" : 1
      }
    }, {
      "movement" : {
        "start" : "1B",
        "end" : "2B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : "r_adv_force",
        "runner" : {
          "id" : 516658,
          "fullName" : "Marcos Martinez",
          "link" : "/api/v1/people/516658"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 1
      }
    }, {
      "movement" : {
        "start" : "2B",
        "end" : "3B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : "r_adv_play",
        "runner" : {
          "id" : 516658,
          "fullName" : "Marcos Martinez",
          "link" : "/api/v1/people/516658"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 1
      }
    }, {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : null,
        "runner" : {
          "id" : 542458,
          "fullName" : "Wader Perez",
          "link" : "/api/v1/people/542458"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 1
      },
      "credits" : [ {
        "player" : {
          "id" : 542758,
          "link" : "/api/v1/people/542758"
        },
        "position" : {
          "code" : "9",
          "name" : "Outfielder",
          "type" : "Outfielder",
          "abbreviation" : "RF"
        },
        "credit" : "f_fielded_ball"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "description" : "Pitcher Change: Gabirel Arias replaces Raudy Sandoval.",
        "event" : "Pitching Substitution",
        "awayScore" : 2,
        "homeScore" : 4,
        "isScoringPlay" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0,
        "outs" : 2
      },
      "index" : 0,
      "isPitch" : false,
      "type" : "action",
      "player" : {
        "id" : 525734,
        "link" : "/api/v1/people/525734"
      },
      "position" : {
        "code" : "1",
        "name" : "Pitcher",
        "type" : "Pitcher",
        "abbreviation" : "P"
      }
    }, {
      "details" : {
        "call" : {
          "code" : "E",
          "description" : "Hit Into Play - Run(s)"
        },
        "description" : "In play, run(s)",
        "code" : "E",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 98.71,
          "y" : 126.93
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "line_drive",
        "hardness" : "medium",
        "location" : "9",
        "coordinates" : {
          "coordX" : 172.69,
          "coordY" : 111.45
        }
      },
      "index" : 1,
      "playId" : "02401986-0546-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 53
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Strikeout",
      "eventType" : "strikeout",
      "description" : "Alexander Castellano strikes out swinging.",
      "rbi" : 0,
      "awayScore" : 4,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 54,
      "halfInning" : "top",
      "inning" : 7,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 14
    },
    "count" : {
      "balls" : 0,
      "strikes" : 3,
      "outs" : 3
    },
    "matchup" : {
      "batter" : {
        "id" : 542463,
        "fullName" : "Alex Castellano",
        "link" : "/api/v1/people/542463"
      },
      "batSide" : {
        "code" : "L",
        "description" : "Left"
      },
      "pitcher" : {
        "id" : 525734,
        "fullName" : "Gabriel Arias",
        "link" : "/api/v1/people/525734"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_LHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0, 1, 2 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 3
      },
      "details" : {
        "event" : "Strikeout",
        "eventType" : "strikeout",
        "movementReason" : null,
        "runner" : {
          "id" : 542463,
          "fullName" : "Alex Castellano",
          "link" : "/api/v1/people/542463"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 2
      },
      "credits" : [ {
        "player" : {
          "id" : 501517,
          "link" : "/api/v1/people/501517"
        },
        "position" : {
          "code" : "2",
          "name" : "Catcher",
          "type" : "Catcher",
          "abbreviation" : "C"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 1
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 89.27,
          "y" : 114.84
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0556-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "C",
          "description" : "Strike - Called"
        },
        "description" : "Called Strike",
        "code" : "C",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 2
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 94.42,
          "y" : 119.16
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0556-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 3
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 96.14,
          "y" : 113.11
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0556-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 54
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Walk",
      "eventType" : "walk",
      "description" : "Eduard Martinez walks.",
      "rbi" : 0,
      "awayScore" : 4,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 55,
      "halfInning" : "bottom",
      "inning" : 7,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 4,
      "strikes" : 0,
      "outs" : 0
    },
    "matchup" : {
      "batter" : {
        "id" : 499960,
        "fullName" : "Eduard Martinez",
        "link" : "/api/v1/people/499960"
      },
      "batSide" : {
        "code" : "L",
        "description" : "Left"
      },
      "pitcher" : {
        "id" : 542246,
        "fullName" : "Randy Santos",
        "link" : "/api/v1/people/542246"
      },
      "pitchHand" : {
        "code" : "L",
        "description" : "Left"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_LHP",
        "pitcher" : "vs_LHB",
        "menOnBase" : "Men_On"
      }
    },
    "pitchIndex" : [ 2, 3, 4, 5 ],
    "actionIndex" : [ 0, 1 ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Walk",
        "eventType" : "walk",
        "movementReason" : null,
        "runner" : {
          "id" : 499960,
          "fullName" : "Eduard Martinez",
          "link" : "/api/v1/people/499960"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 5
      },
      "credits" : [ ]
    } ],
    "playEvents" : [ {
      "details" : {
        "description" : "Offensive Substitution: Pinch hitter Eduard Martinez replaces Carlos Best.",
        "event" : "Offensive Substitution",
        "eventType" : "offensive_substitution",
        "awayScore" : 4,
        "homeScore" : 4,
        "isScoringPlay" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0,
        "outs" : 0
      },
      "index" : 0,
      "isPitch" : false,
      "type" : "action",
      "player" : {
        "id" : 499960,
        "link" : "/api/v1/people/499960"
      },
      "battingOrder" : "901"
    }, {
      "details" : {
        "description" : "Pitcher Change: Randy Santos replaces Eddy Rivera.",
        "event" : "Pitching Substitution",
        "awayScore" : 4,
        "homeScore" : 4,
        "isScoringPlay" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0,
        "outs" : 0
      },
      "index" : 1,
      "isPitch" : false,
      "type" : "action",
      "player" : {
        "id" : 542246,
        "link" : "/api/v1/people/542246"
      },
      "position" : {
        "code" : "1",
        "name" : "Pitcher",
        "type" : "Pitcher",
        "abbreviation" : "P"
      }
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 126.18,
          "y" : 114.84
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0566-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 2,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 117.6,
          "y" : 108.8
        },
        "breaks" : { }
      },
      "index" : 3,
      "playId" : "02401986-0566-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 3,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 121.03,
          "y" : 120.88
        },
        "breaks" : { }
      },
      "index" : 4,
      "playId" : "02401986-0566-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 4,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 123.61,
          "y" : 109.66
        },
        "breaks" : { }
      },
      "index" : 5,
      "playId" : "02401986-0566-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 55
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Single",
      "eventType" : "single",
      "description" : "Jonathan Villan singles on a line drive to left fielder Santo Sandoval.    Eduard Martinez to 2nd.",
      "rbi" : 0,
      "awayScore" : 4,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 56,
      "halfInning" : "bottom",
      "inning" : 7,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 33
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 0
    },
    "matchup" : {
      "batter" : {
        "id" : 542340,
        "fullName" : "Jonathan Villar",
        "link" : "/api/v1/people/542340"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542246,
        "fullName" : "Randy Santos",
        "link" : "/api/v1/people/542246"
      },
      "pitchHand" : {
        "code" : "L",
        "description" : "Left"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_LHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "RISP"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0, 1 ],
    "runners" : [ {
      "movement" : {
        "start" : "1B",
        "end" : "2B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : "r_adv_force",
        "runner" : {
          "id" : 499960,
          "fullName" : "Eduard Martinez",
          "link" : "/api/v1/people/499960"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      }
    }, {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : null,
        "runner" : {
          "id" : 542340,
          "fullName" : "Jonathan Villar",
          "link" : "/api/v1/people/542340"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 542468,
          "link" : "/api/v1/people/542468"
        },
        "position" : {
          "code" : "7",
          "name" : "Outfielder",
          "type" : "Outfielder",
          "abbreviation" : "LF"
        },
        "credit" : "f_fielded_ball"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "D",
          "description" : "Hit Into Play - No Out(s)"
        },
        "description" : "In play, no out",
        "code" : "D",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 89.27,
          "y" : 120.02
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "line_drive",
        "hardness" : "medium",
        "location" : "7",
        "coordinates" : {
          "coordX" : 69.28,
          "coordY" : 122.49
        }
      },
      "index" : 0,
      "playId" : "02401986-0576-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 56
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Forceout",
      "eventType" : "force_out",
      "description" : "Carlos Valenzuela grounds into a force out, first baseman Marcos Martinez to third baseman Luis Pimentel.    Eduard Martinez out at 3rd.    Jonathan Villan to 2nd.    Carlos Valenzuela to 1st.",
      "rbi" : 0,
      "awayScore" : 4,
      "homeScore" : 4
    },
    "about" : {
      "atBatIndex" : 57,
      "halfInning" : "bottom",
      "inning" : 7,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 1
    },
    "matchup" : {
      "batter" : {
        "id" : 542477,
        "fullName" : "Carlos Valenzuela",
        "link" : "/api/v1/people/542477"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542246,
        "fullName" : "Randy Santos",
        "link" : "/api/v1/people/542246"
      },
      "pitchHand" : {
        "code" : "L",
        "description" : "Left"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_LHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "RISP"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0, 1, 2 ],
    "runners" : [ {
      "movement" : {
        "start" : "1B",
        "end" : "2B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Forceout",
        "eventType" : "force_out",
        "movementReason" : "r_adv_play",
        "runner" : {
          "id" : 542340,
          "fullName" : "Jonathan Villar",
          "link" : "/api/v1/people/542340"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      }
    }, {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Forceout",
        "eventType" : "force_out",
        "movementReason" : null,
        "runner" : {
          "id" : 542477,
          "fullName" : "Carlos Valenzuela",
          "link" : "/api/v1/people/542477"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ ]
    }, {
      "movement" : {
        "start" : "2B",
        "end" : null,
        "outBase" : "3B",
        "isOut" : true,
        "outNumber" : 1
      },
      "details" : {
        "event" : "Forceout",
        "eventType" : "force_out",
        "movementReason" : "r_runner_out",
        "runner" : {
          "id" : 499960,
          "fullName" : "Eduard Martinez",
          "link" : "/api/v1/people/499960"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 516658,
          "link" : "/api/v1/people/516658"
        },
        "position" : {
          "code" : "3",
          "name" : "First Base",
          "type" : "Infielder",
          "abbreviation" : "1B"
        },
        "credit" : "f_assist"
      }, {
        "player" : {
          "id" : 540938,
          "link" : "/api/v1/people/540938"
        },
        "position" : {
          "code" : "5",
          "name" : "Third Base",
          "type" : "Infielder",
          "abbreviation" : "3B"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 98.71,
          "y" : 116.57
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "ground_ball",
        "hardness" : "medium",
        "location" : "3",
        "coordinates" : {
          "coordX" : 144.58,
          "coordY" : 176.71
        }
      },
      "index" : 0,
      "playId" : "02401986-0586-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 57
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Single",
      "eventType" : "single",
      "description" : "Miguel Alvarez singles on a fly ball to center fielder Roberto Reyes.    Jonathan Villan scores.    Carlos Valenzuela to 3rd.",
      "rbi" : 1,
      "awayScore" : 4,
      "homeScore" : 5
    },
    "about" : {
      "atBatIndex" : 58,
      "halfInning" : "bottom",
      "inning" : 7,
      "isComplete" : true,
      "isScoringPlay" : true,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 60
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 1
    },
    "matchup" : {
      "batter" : {
        "id" : 516758,
        "fullName" : "Miguel Alvarez",
        "link" : "/api/v1/people/516758"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542246,
        "fullName" : "Randy Santos",
        "link" : "/api/v1/people/542246"
      },
      "pitchHand" : {
        "code" : "L",
        "description" : "Left"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_LHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "RISP"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0, 1, 2, 3, 4 ],
    "runners" : [ {
      "movement" : {
        "start" : "2B",
        "end" : "3B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : "r_adv_force",
        "runner" : {
          "id" : 542340,
          "fullName" : "Jonathan Villar",
          "link" : "/api/v1/people/542340"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      }
    }, {
      "movement" : {
        "start" : "3B",
        "end" : "score",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : "r_adv_play",
        "runner" : {
          "id" : 542340,
          "fullName" : "Jonathan Villar",
          "link" : "/api/v1/people/542340"
        },
        "responsiblePitcher" : {
          "id" : 542246,
          "link" : "/api/v1/people/542246"
        },
        "isScoringEvent" : true,
        "rbi" : true,
        "earned" : true,
        "teamUnearned" : false,
        "playIndex" : 0
      }
    }, {
      "movement" : {
        "start" : "1B",
        "end" : "2B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : "r_adv_force",
        "runner" : {
          "id" : 542477,
          "fullName" : "Carlos Valenzuela",
          "link" : "/api/v1/people/542477"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      }
    }, {
      "movement" : {
        "start" : "2B",
        "end" : "3B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : "r_adv_play",
        "runner" : {
          "id" : 542477,
          "fullName" : "Carlos Valenzuela",
          "link" : "/api/v1/people/542477"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      }
    }, {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : null,
        "runner" : {
          "id" : 516758,
          "fullName" : "Miguel Alvarez",
          "link" : "/api/v1/people/516758"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 542465,
          "link" : "/api/v1/people/542465"
        },
        "position" : {
          "code" : "8",
          "name" : "Outfielder",
          "type" : "Outfielder",
          "abbreviation" : "CF"
        },
        "credit" : "f_fielded_ball"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "E",
          "description" : "Hit Into Play - Run(s)"
        },
        "description" : "In play, run(s)",
        "code" : "E",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 97.0,
          "y" : 116.57
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "fly_ball",
        "hardness" : "medium",
        "location" : "8",
        "coordinates" : {
          "coordX" : 124.5,
          "coordY" : 104.42
        }
      },
      "index" : 0,
      "playId" : "02401986-0596-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 58
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Pop Out",
      "eventType" : "field_out",
      "description" : "Pedro Aguilar pops out to shortstop Juan B Cabrera.",
      "rbi" : 0,
      "awayScore" : 4,
      "homeScore" : 6
    },
    "about" : {
      "atBatIndex" : 59,
      "halfInning" : "bottom",
      "inning" : 7,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 2,
      "strikes" : 0,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 516729,
        "fullName" : "Pedro Aguilar",
        "link" : "/api/v1/people/516729"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542246,
        "fullName" : "Randy Santos",
        "link" : "/api/v1/people/542246"
      },
      "pitchHand" : {
        "code" : "L",
        "description" : "Left"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_LHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "RISP"
      }
    },
    "pitchIndex" : [ 0, 2, 4 ],
    "actionIndex" : [ 1, 3 ],
    "runnerIndex" : [ 2 ],
    "runners" : [ {
      "movement" : {
        "start" : "1B",
        "end" : "2B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Stolen Base 2B",
        "eventType" : "stolen_base_2b",
        "movementReason" : "r_stolen_base_2b",
        "runner" : {
          "id" : 516758,
          "fullName" : "Miguel Alvarez",
          "link" : "/api/v1/people/516758"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 1
      },
      "credits" : [ ]
    }, {
      "movement" : {
        "start" : "3B",
        "end" : "score",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Wild Pitch",
        "eventType" : "wild_pitch",
        "movementReason" : "r_adv_play",
        "runner" : {
          "id" : 542477,
          "fullName" : "Carlos Valenzuela",
          "link" : "/api/v1/people/542477"
        },
        "responsiblePitcher" : {
          "id" : 542246,
          "link" : "/api/v1/people/542246"
        },
        "isScoringEvent" : true,
        "rbi" : false,
        "earned" : true,
        "teamUnearned" : false,
        "playIndex" : 3
      },
      "credits" : [ ]
    }, {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 2
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : null,
        "runner" : {
          "id" : 516729,
          "fullName" : "Pedro Aguilar",
          "link" : "/api/v1/people/516729"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 4
      },
      "credits" : [ {
        "player" : {
          "id" : 516868,
          "link" : "/api/v1/people/516868"
        },
        "position" : {
          "code" : "6",
          "name" : "Shortstop",
          "type" : "Infielder",
          "abbreviation" : "SS"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 60.94,
          "y" : 119.16
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0606-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "description" : "With Pedro Aguilar batting, Miguel Alvarez steals (18) 2nd base.",
        "event" : "Stolen Base 2B",
        "eventType" : "stolen_base_2b",
        "awayScore" : 4,
        "homeScore" : 5,
        "isScoringPlay" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0,
        "outs" : 1
      },
      "index" : 1,
      "isPitch" : false,
      "type" : "action",
      "player" : {
        "id" : 516758,
        "link" : "/api/v1/people/516758"
      }
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 2,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 54.94,
          "y" : 101.89
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0606-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "description" : "With Pedro Aguilar batting, wild pitch by Randy Santos, Carlos Valenzuela scores.",
        "event" : "Wild Pitch",
        "eventType" : "wild_pitch",
        "awayScore" : 4,
        "homeScore" : 6,
        "isScoringPlay" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 2,
        "strikes" : 0,
        "outs" : 1
      },
      "index" : 3,
      "isPitch" : false,
      "type" : "action",
      "player" : {
        "id" : 542233,
        "link" : "/api/v1/people/542233"
      }
    }, {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 2,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 90.99,
          "y" : 125.2
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "popup",
        "hardness" : "medium",
        "location" : "6",
        "coordinates" : {
          "coordX" : 96.39,
          "coordY" : 146.59
        }
      },
      "index" : 4,
      "playId" : "02401986-0606-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 59
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Walk",
      "eventType" : "walk",
      "description" : "Geancarlo Mendez walks.",
      "rbi" : 0,
      "awayScore" : 4,
      "homeScore" : 6
    },
    "about" : {
      "atBatIndex" : 60,
      "halfInning" : "bottom",
      "inning" : 7,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 4,
      "strikes" : 0,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 542479,
        "fullName" : "Geancarlo Mendez",
        "link" : "/api/v1/people/542479"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542246,
        "fullName" : "Randy Santos",
        "link" : "/api/v1/people/542246"
      },
      "pitchHand" : {
        "code" : "L",
        "description" : "Left"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_LHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "RISP"
      }
    },
    "pitchIndex" : [ 0, 1, 2, 3 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Walk",
        "eventType" : "walk",
        "movementReason" : null,
        "runner" : {
          "id" : 542479,
          "fullName" : "Geancarlo Mendez",
          "link" : "/api/v1/people/542479"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 3
      },
      "credits" : [ ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 54.08,
          "y" : 107.07
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0616-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 2,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 41.2,
          "y" : 98.43
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0616-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 3,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 44.64,
          "y" : 98.43
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0616-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 4,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 49.79,
          "y" : 98.43
        },
        "breaks" : { }
      },
      "index" : 3,
      "playId" : "02401986-0616-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 60
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Walk",
      "eventType" : "walk",
      "description" : "Emmanuel Checo walks.    Miguel Alvarez to 3rd.    Geancarlo Mendez to 2nd.",
      "rbi" : 0,
      "awayScore" : 4,
      "homeScore" : 6
    },
    "about" : {
      "atBatIndex" : 61,
      "halfInning" : "bottom",
      "inning" : 7,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 4,
      "strikes" : 0,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 501517,
        "fullName" : "Emmanuel Checo",
        "link" : "/api/v1/people/501517"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542246,
        "fullName" : "Randy Santos",
        "link" : "/api/v1/people/542246"
      },
      "pitchHand" : {
        "code" : "L",
        "description" : "Left"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_LHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Loaded"
      }
    },
    "pitchIndex" : [ 0, 1, 2, 3 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0, 1, 2 ],
    "runners" : [ {
      "movement" : {
        "start" : "2B",
        "end" : "3B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Walk",
        "eventType" : "walk",
        "movementReason" : "r_adv_force",
        "runner" : {
          "id" : 516758,
          "fullName" : "Miguel Alvarez",
          "link" : "/api/v1/people/516758"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 3
      }
    }, {
      "movement" : {
        "start" : "1B",
        "end" : "2B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Walk",
        "eventType" : "walk",
        "movementReason" : "r_adv_force",
        "runner" : {
          "id" : 542479,
          "fullName" : "Geancarlo Mendez",
          "link" : "/api/v1/people/542479"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 3
      }
    }, {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Walk",
        "eventType" : "walk",
        "movementReason" : null,
        "runner" : {
          "id" : 501517,
          "fullName" : "Emmanuel Checo",
          "link" : "/api/v1/people/501517"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 3
      },
      "credits" : [ ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 58.37,
          "y" : 102.75
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0626-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 2,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 44.64,
          "y" : 104.48
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0626-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 3,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 56.65,
          "y" : 109.66
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0626-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 4,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 54.94,
          "y" : 102.75
        },
        "breaks" : { }
      },
      "index" : 3,
      "playId" : "02401986-0626-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 61
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Strikeout",
      "eventType" : "strikeout",
      "description" : "Luis Paulino strikes out swinging.",
      "rbi" : 0,
      "awayScore" : 4,
      "homeScore" : 6
    },
    "about" : {
      "atBatIndex" : 62,
      "halfInning" : "bottom",
      "inning" : 7,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 14
    },
    "count" : {
      "balls" : 0,
      "strikes" : 3,
      "outs" : 3
    },
    "matchup" : {
      "batter" : {
        "id" : 516760,
        "fullName" : "Luis Paulino",
        "link" : "/api/v1/people/516760"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542523,
        "fullName" : "Amaury Castillo",
        "link" : "/api/v1/people/542523"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 1, 2, 3 ],
    "actionIndex" : [ 0 ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 3
      },
      "details" : {
        "event" : "Strikeout",
        "eventType" : "strikeout",
        "movementReason" : null,
        "runner" : {
          "id" : 516760,
          "fullName" : "Luis Paulino",
          "link" : "/api/v1/people/516760"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 3
      },
      "credits" : [ {
        "player" : {
          "id" : 542233,
          "link" : "/api/v1/people/542233"
        },
        "position" : {
          "code" : "2",
          "name" : "Catcher",
          "type" : "Catcher",
          "abbreviation" : "C"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "description" : "Pitcher Change: Amaury Castillo replaces Randy Santos.",
        "event" : "Pitching Substitution",
        "awayScore" : 4,
        "homeScore" : 6,
        "isScoringPlay" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0,
        "outs" : 2
      },
      "index" : 0,
      "isPitch" : false,
      "type" : "action",
      "player" : {
        "id" : 542523,
        "link" : "/api/v1/people/542523"
      },
      "position" : {
        "code" : "1",
        "name" : "Pitcher",
        "type" : "Pitcher",
        "abbreviation" : "P"
      }
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 1
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 96.14,
          "y" : 126.93
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0636-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 2
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 97.0,
          "y" : 124.34
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0636-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 3
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 91.85,
          "y" : 117.43
        },
        "breaks" : { }
      },
      "index" : 3,
      "playId" : "02401986-0636-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 62
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Walk",
      "eventType" : "walk",
      "description" : "Juan B Cabrera walks.",
      "rbi" : 0,
      "awayScore" : 4,
      "homeScore" : 6
    },
    "about" : {
      "atBatIndex" : 63,
      "halfInning" : "top",
      "inning" : 8,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 4,
      "strikes" : 0,
      "outs" : 0
    },
    "matchup" : {
      "batter" : {
        "id" : 516868,
        "fullName" : "Juan B Cabrera",
        "link" : "/api/v1/people/516868"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 525734,
        "fullName" : "Gabriel Arias",
        "link" : "/api/v1/people/525734"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Men_On"
      }
    },
    "pitchIndex" : [ 1, 2, 3, 4 ],
    "actionIndex" : [ 0 ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Walk",
        "eventType" : "walk",
        "movementReason" : null,
        "runner" : {
          "id" : 516868,
          "fullName" : "Juan B Cabrera",
          "link" : "/api/v1/people/516868"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 4
      },
      "credits" : [ ]
    } ],
    "playEvents" : [ {
      "details" : {
        "description" : "Eduard Martinez remains in the game as the right fielder.",
        "event" : "Defensive Switch",
        "eventType" : "defensive_switch",
        "awayScore" : 4,
        "homeScore" : 6,
        "isScoringPlay" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0,
        "outs" : 0
      },
      "index" : 0,
      "isPitch" : false,
      "type" : "action",
      "player" : {
        "id" : 499960,
        "link" : "/api/v1/people/499960"
      },
      "battingOrder" : "901"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 54.94,
          "y" : 107.93
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0646-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 2,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 47.21,
          "y" : 107.93
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0646-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 3,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 50.64,
          "y" : 106.2
        },
        "breaks" : { }
      },
      "index" : 3,
      "playId" : "02401986-0646-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 4,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 59.23,
          "y" : 97.57
        },
        "breaks" : { }
      },
      "index" : 4,
      "playId" : "02401986-0646-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 63
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Groundout",
      "eventType" : "field_out",
      "description" : "Audris Perez grounds out, shortstop Jonathan Villan to first baseman Geancarlo Mendez.",
      "rbi" : 0,
      "awayScore" : 4,
      "homeScore" : 6
    },
    "about" : {
      "atBatIndex" : 64,
      "halfInning" : "top",
      "inning" : 8,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 1,
      "strikes" : 0,
      "outs" : 1
    },
    "matchup" : {
      "batter" : {
        "id" : 542233,
        "fullName" : "Audry Perez",
        "link" : "/api/v1/people/542233"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 525734,
        "fullName" : "Gabriel Arias",
        "link" : "/api/v1/people/525734"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "RISP"
      }
    },
    "pitchIndex" : [ 0, 2 ],
    "actionIndex" : [ 1 ],
    "runnerIndex" : [ 1 ],
    "runners" : [ {
      "movement" : {
        "start" : "1B",
        "end" : "2B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Stolen Base 2B",
        "eventType" : "stolen_base_2b",
        "movementReason" : "r_stolen_base_2b",
        "runner" : {
          "id" : 516868,
          "fullName" : "Juan B Cabrera",
          "link" : "/api/v1/people/516868"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 1
      },
      "credits" : [ ]
    }, {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 1
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : null,
        "runner" : {
          "id" : 542233,
          "fullName" : "Audry Perez",
          "link" : "/api/v1/people/542233"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 2
      },
      "credits" : [ {
        "player" : {
          "id" : 542340,
          "link" : "/api/v1/people/542340"
        },
        "position" : {
          "code" : "6",
          "name" : "Shortstop",
          "type" : "Infielder",
          "abbreviation" : "SS"
        },
        "credit" : "f_assist"
      }, {
        "player" : {
          "id" : 542479,
          "link" : "/api/v1/people/542479"
        },
        "position" : {
          "code" : "3",
          "name" : "First Base",
          "type" : "Infielder",
          "abbreviation" : "1B"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 66.95,
          "y" : 121.75
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0656-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "description" : "With Audris Perez batting, Juan B Cabrera steals (7) 2nd base.",
        "event" : "Stolen Base 2B",
        "eventType" : "stolen_base_2b",
        "awayScore" : 4,
        "homeScore" : 6,
        "isScoringPlay" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0,
        "outs" : 0
      },
      "index" : 1,
      "isPitch" : false,
      "type" : "action",
      "player" : {
        "id" : 516868,
        "link" : "/api/v1/people/516868"
      }
    }, {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 93.56,
          "y" : 120.02
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "ground_ball",
        "hardness" : "medium",
        "location" : "6",
        "coordinates" : {
          "coordX" : 106.43,
          "coordY" : 165.66
        }
      },
      "index" : 2,
      "playId" : "02401986-0656-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 64
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Flyout",
      "eventType" : "field_out",
      "description" : "Luis Pimentel flies out to center fielder Miguel Alvarez.",
      "rbi" : 0,
      "awayScore" : 4,
      "homeScore" : 6
    },
    "about" : {
      "atBatIndex" : 65,
      "halfInning" : "top",
      "inning" : 8,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 540938,
        "fullName" : "Luis Pimentel",
        "link" : "/api/v1/people/540938"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 525734,
        "fullName" : "Gabriel Arias",
        "link" : "/api/v1/people/525734"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "RISP"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 2
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : null,
        "runner" : {
          "id" : 540938,
          "fullName" : "Luis Pimentel",
          "link" : "/api/v1/people/540938"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 516758,
          "link" : "/api/v1/people/516758"
        },
        "position" : {
          "code" : "8",
          "name" : "Outfielder",
          "type" : "Outfielder",
          "abbreviation" : "CF"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 88.41,
          "y" : 121.75
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "fly_ball",
        "hardness" : "medium",
        "location" : "8",
        "coordinates" : {
          "coordX" : 120.48,
          "coordY" : 107.43
        }
      },
      "index" : 0,
      "playId" : "02401986-0666-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 65
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Strikeout",
      "eventType" : "strikeout",
      "description" : "Santo Sandoval called out on strikes.",
      "rbi" : 0,
      "awayScore" : 4,
      "homeScore" : 6
    },
    "about" : {
      "atBatIndex" : 66,
      "halfInning" : "top",
      "inning" : 8,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 14
    },
    "count" : {
      "balls" : 0,
      "strikes" : 3,
      "outs" : 3
    },
    "matchup" : {
      "batter" : {
        "id" : 542468,
        "fullName" : "Santo Sandoval",
        "link" : "/api/v1/people/542468"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 525734,
        "fullName" : "Gabriel Arias",
        "link" : "/api/v1/people/525734"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0, 1, 2 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 3
      },
      "details" : {
        "event" : "Strikeout",
        "eventType" : "strikeout",
        "movementReason" : null,
        "runner" : {
          "id" : 542468,
          "fullName" : "Santo Sandoval",
          "link" : "/api/v1/people/542468"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 2
      },
      "credits" : [ {
        "player" : {
          "id" : 501517,
          "link" : "/api/v1/people/501517"
        },
        "position" : {
          "code" : "2",
          "name" : "Catcher",
          "type" : "Catcher",
          "abbreviation" : "C"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "C",
          "description" : "Strike - Called"
        },
        "description" : "Called Strike",
        "code" : "C",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 1
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 90.13,
          "y" : 126.06
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0676-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "C",
          "description" : "Strike - Called"
        },
        "description" : "Called Strike",
        "code" : "C",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 2
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 92.7,
          "y" : 126.93
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0676-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "C",
          "description" : "Strike - Called"
        },
        "description" : "Called Strike",
        "code" : "C",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 3
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 85.84,
          "y" : 121.75
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0676-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 66
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Strikeout",
      "eventType" : "strikeout",
      "description" : "Jose Trinidad called out on strikes.",
      "rbi" : 0,
      "awayScore" : 4,
      "homeScore" : 6
    },
    "about" : {
      "atBatIndex" : 67,
      "halfInning" : "bottom",
      "inning" : 8,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 14
    },
    "count" : {
      "balls" : 0,
      "strikes" : 3,
      "outs" : 1
    },
    "matchup" : {
      "batter" : {
        "id" : 542339,
        "fullName" : "Jose Trinidad",
        "link" : "/api/v1/people/542339"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542523,
        "fullName" : "Amaury Castillo",
        "link" : "/api/v1/people/542523"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0, 1, 2 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 1
      },
      "details" : {
        "event" : "Strikeout",
        "eventType" : "strikeout",
        "movementReason" : null,
        "runner" : {
          "id" : 542339,
          "fullName" : "Jose Trinidad",
          "link" : "/api/v1/people/542339"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 2
      },
      "credits" : [ {
        "player" : {
          "id" : 542233,
          "link" : "/api/v1/people/542233"
        },
        "position" : {
          "code" : "2",
          "name" : "Catcher",
          "type" : "Catcher",
          "abbreviation" : "C"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "C",
          "description" : "Strike - Called"
        },
        "description" : "Called Strike",
        "code" : "C",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 1
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 94.42,
          "y" : 126.06
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0686-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "C",
          "description" : "Strike - Called"
        },
        "description" : "Called Strike",
        "code" : "C",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 2
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 91.85,
          "y" : 125.2
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0686-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "C",
          "description" : "Strike - Called"
        },
        "description" : "Called Strike",
        "code" : "C",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 3
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 92.7,
          "y" : 121.75
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0686-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 67
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Walk",
      "eventType" : "walk",
      "description" : "Eduard Martinez walks.",
      "rbi" : 0,
      "awayScore" : 4,
      "homeScore" : 6
    },
    "about" : {
      "atBatIndex" : 68,
      "halfInning" : "bottom",
      "inning" : 8,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 4,
      "strikes" : 0,
      "outs" : 1
    },
    "matchup" : {
      "batter" : {
        "id" : 499960,
        "fullName" : "Eduard Martinez",
        "link" : "/api/v1/people/499960"
      },
      "batSide" : {
        "code" : "L",
        "description" : "Left"
      },
      "pitcher" : {
        "id" : 542523,
        "fullName" : "Amaury Castillo",
        "link" : "/api/v1/people/542523"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_LHB",
        "menOnBase" : "Men_On"
      }
    },
    "pitchIndex" : [ 0, 1, 2, 3 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Walk",
        "eventType" : "walk",
        "movementReason" : null,
        "runner" : {
          "id" : 499960,
          "fullName" : "Eduard Martinez",
          "link" : "/api/v1/people/499960"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 3
      },
      "credits" : [ ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 126.18,
          "y" : 122.61
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0696-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 2,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 128.76,
          "y" : 120.88
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0696-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 3,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 119.31,
          "y" : 125.2
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0696-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 4,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 118.45,
          "y" : 119.16
        },
        "breaks" : { }
      },
      "index" : 3,
      "playId" : "02401986-0696-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 68
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Walk",
      "eventType" : "walk",
      "description" : "Jonathan Villan walks.",
      "rbi" : 0,
      "awayScore" : 4,
      "homeScore" : 6
    },
    "about" : {
      "atBatIndex" : 69,
      "halfInning" : "bottom",
      "inning" : 8,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 4,
      "strikes" : 0,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 542340,
        "fullName" : "Jonathan Villar",
        "link" : "/api/v1/people/542340"
      },
      "batSide" : {
        "code" : "L",
        "description" : "Left"
      },
      "pitcher" : {
        "id" : 542523,
        "fullName" : "Amaury Castillo",
        "link" : "/api/v1/people/542523"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_LHB",
        "menOnBase" : "Men_On"
      }
    },
    "pitchIndex" : [ 0, 2, 3, 4 ],
    "actionIndex" : [ 1 ],
    "runnerIndex" : [ 1 ],
    "runners" : [ {
      "movement" : {
        "start" : "1B",
        "end" : null,
        "outBase" : "2B",
        "isOut" : true,
        "outNumber" : 2
      },
      "details" : {
        "event" : "Caught Stealing 2B",
        "eventType" : "caught_stealing_2b",
        "movementReason" : "r_caught_stealing_2b",
        "runner" : {
          "id" : 499960,
          "fullName" : "Eduard Martinez",
          "link" : "/api/v1/people/499960"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 1
      },
      "credits" : [ {
        "player" : {
          "id" : 542233,
          "link" : "/api/v1/people/542233"
        },
        "position" : {
          "code" : "2",
          "name" : "Catcher",
          "type" : "Catcher",
          "abbreviation" : "C"
        },
        "credit" : "f_assist"
      }, {
        "player" : {
          "id" : 516868,
          "link" : "/api/v1/people/516868"
        },
        "position" : {
          "code" : "6",
          "name" : "Shortstop",
          "type" : "Infielder",
          "abbreviation" : "SS"
        },
        "credit" : "f_putout"
      } ]
    }, {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Walk",
        "eventType" : "walk",
        "movementReason" : null,
        "runner" : {
          "id" : 542340,
          "fullName" : "Jonathan Villar",
          "link" : "/api/v1/people/542340"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 4
      },
      "credits" : [ ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 122.75,
          "y" : 124.34
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0706-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "description" : "With Jonathan Villan batting, Eduard Martinez caught stealing 2nd base, catcher Audris Perez to shortstop Juan B Cabrera.",
        "event" : "Caught Stealing 2B",
        "eventType" : "caught_stealing_2b",
        "awayScore" : 4,
        "homeScore" : 6,
        "isScoringPlay" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0,
        "outs" : 2
      },
      "index" : 1,
      "isPitch" : false,
      "type" : "action",
      "player" : {
        "id" : 499960,
        "link" : "/api/v1/people/499960"
      }
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 2,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 122.75,
          "y" : 120.02
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0706-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 3,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 127.9,
          "y" : 116.57
        },
        "breaks" : { }
      },
      "index" : 3,
      "playId" : "02401986-0706-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 4,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 121.03,
          "y" : 119.16
        },
        "breaks" : { }
      },
      "index" : 4,
      "playId" : "02401986-0706-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 69
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Single",
      "eventType" : "single",
      "description" : "Carlos Valenzuela singles on a line drive to left fielder Santo Sandoval.    Jonathan Villan scores.",
      "rbi" : 1,
      "awayScore" : 4,
      "homeScore" : 7
    },
    "about" : {
      "atBatIndex" : 70,
      "halfInning" : "bottom",
      "inning" : 8,
      "isComplete" : true,
      "isScoringPlay" : true,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 70
    },
    "count" : {
      "balls" : 2,
      "strikes" : 0,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 542477,
        "fullName" : "Carlos Valenzuela",
        "link" : "/api/v1/people/542477"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542523,
        "fullName" : "Amaury Castillo",
        "link" : "/api/v1/people/542523"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Men_On"
      }
    },
    "pitchIndex" : [ 0, 2, 4 ],
    "actionIndex" : [ 1, 3 ],
    "runnerIndex" : [ 2, 3 ],
    "runners" : [ {
      "movement" : {
        "start" : "1B",
        "end" : "2B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Stolen Base 2B",
        "eventType" : "stolen_base_2b",
        "movementReason" : "r_stolen_base_2b",
        "runner" : {
          "id" : 542340,
          "fullName" : "Jonathan Villar",
          "link" : "/api/v1/people/542340"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 1
      },
      "credits" : [ ]
    }, {
      "movement" : {
        "start" : "2B",
        "end" : "3B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Passed Ball",
        "eventType" : "passed_ball",
        "movementReason" : "r_adv_play",
        "runner" : {
          "id" : 542340,
          "fullName" : "Jonathan Villar",
          "link" : "/api/v1/people/542340"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 3
      },
      "credits" : [ ]
    }, {
      "movement" : {
        "start" : "3B",
        "end" : "score",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : "r_adv_play",
        "runner" : {
          "id" : 542340,
          "fullName" : "Jonathan Villar",
          "link" : "/api/v1/people/542340"
        },
        "responsiblePitcher" : {
          "id" : 542523,
          "link" : "/api/v1/people/542523"
        },
        "isScoringEvent" : true,
        "rbi" : true,
        "earned" : true,
        "teamUnearned" : false,
        "playIndex" : 4
      }
    }, {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : null,
        "runner" : {
          "id" : 542477,
          "fullName" : "Carlos Valenzuela",
          "link" : "/api/v1/people/542477"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 4
      },
      "credits" : [ {
        "player" : {
          "id" : 542468,
          "link" : "/api/v1/people/542468"
        },
        "position" : {
          "code" : "7",
          "name" : "Outfielder",
          "type" : "Outfielder",
          "abbreviation" : "LF"
        },
        "credit" : "f_fielded_ball"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 53.22,
          "y" : 105.34
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0716-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "description" : "With Carlos Valenzuela batting, Jonathan Villan steals (13) 2nd base.",
        "event" : "Stolen Base 2B",
        "eventType" : "stolen_base_2b",
        "awayScore" : 4,
        "homeScore" : 6,
        "isScoringPlay" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0,
        "outs" : 2
      },
      "index" : 1,
      "isPitch" : false,
      "type" : "action",
      "player" : {
        "id" : 542340,
        "link" : "/api/v1/people/542340"
      }
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 2,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 54.08,
          "y" : 120.88
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0716-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "description" : "With Carlos Valenzuela batting, passed ball by Audris Perez, Jonathan Villan to 3rd.",
        "event" : "Passed Ball",
        "eventType" : "passed_ball",
        "awayScore" : 4,
        "homeScore" : 6,
        "isScoringPlay" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 2,
        "strikes" : 0,
        "outs" : 2
      },
      "index" : 3,
      "isPitch" : false,
      "type" : "action",
      "player" : {
        "id" : 542233,
        "link" : "/api/v1/people/542233"
      }
    }, {
      "details" : {
        "call" : {
          "code" : "E",
          "description" : "Hit Into Play - Run(s)"
        },
        "description" : "In play, run(s)",
        "code" : "E",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 2,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 91.85,
          "y" : 110.52
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "line_drive",
        "hardness" : "medium",
        "location" : "7",
        "coordinates" : {
          "coordX" : 72.29,
          "coordY" : 118.47
        }
      },
      "index" : 4,
      "playId" : "02401986-0716-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 70
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Walk",
      "eventType" : "walk",
      "description" : "Miguel Alvarez walks.    Carlos Valenzuela to 2nd.",
      "rbi" : 0,
      "awayScore" : 4,
      "homeScore" : 7
    },
    "about" : {
      "atBatIndex" : 71,
      "halfInning" : "bottom",
      "inning" : 8,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 4,
      "strikes" : 0,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 516758,
        "fullName" : "Miguel Alvarez",
        "link" : "/api/v1/people/516758"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542523,
        "fullName" : "Amaury Castillo",
        "link" : "/api/v1/people/542523"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "RISP"
      }
    },
    "pitchIndex" : [ 0, 1, 2, 3 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0, 1 ],
    "runners" : [ {
      "movement" : {
        "start" : "1B",
        "end" : "2B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Walk",
        "eventType" : "walk",
        "movementReason" : "r_adv_force",
        "runner" : {
          "id" : 542477,
          "fullName" : "Carlos Valenzuela",
          "link" : "/api/v1/people/542477"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 3
      }
    }, {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Walk",
        "eventType" : "walk",
        "movementReason" : null,
        "runner" : {
          "id" : 516758,
          "fullName" : "Miguel Alvarez",
          "link" : "/api/v1/people/516758"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 3
      },
      "credits" : [ ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 54.08,
          "y" : 101.89
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0726-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 2,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 48.93,
          "y" : 111.39
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0726-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 3,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 60.94,
          "y" : 105.34
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0726-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 4,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 52.36,
          "y" : 107.93
        },
        "breaks" : { }
      },
      "index" : 3,
      "playId" : "02401986-0726-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 71
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Pop Out",
      "eventType" : "field_out",
      "description" : "Pedro Aguilar pops out to third baseman Luis Pimentel.",
      "rbi" : 0,
      "awayScore" : 4,
      "homeScore" : 7
    },
    "about" : {
      "atBatIndex" : 72,
      "halfInning" : "bottom",
      "inning" : 8,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 3
    },
    "matchup" : {
      "batter" : {
        "id" : 516729,
        "fullName" : "Pedro Aguilar",
        "link" : "/api/v1/people/516729"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542523,
        "fullName" : "Amaury Castillo",
        "link" : "/api/v1/people/542523"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 3
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : null,
        "runner" : {
          "id" : 516729,
          "fullName" : "Pedro Aguilar",
          "link" : "/api/v1/people/516729"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 540938,
          "link" : "/api/v1/people/540938"
        },
        "position" : {
          "code" : "5",
          "name" : "Third Base",
          "type" : "Infielder",
          "abbreviation" : "3B"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 97.85,
          "y" : 121.75
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "popup",
        "hardness" : "medium",
        "location" : "5",
        "coordinates" : {
          "coordX" : 103.41,
          "coordY" : 176.71
        }
      },
      "index" : 0,
      "playId" : "02401986-0736-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 72
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Hit By Pitch",
      "eventType" : "hit_by_pitch",
      "description" : "Bernardo Villar hit by pitch.",
      "rbi" : 0,
      "awayScore" : 4,
      "homeScore" : 7
    },
    "about" : {
      "atBatIndex" : 73,
      "halfInning" : "top",
      "inning" : 9,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 11
    },
    "count" : {
      "balls" : 1,
      "strikes" : 0,
      "outs" : 0
    },
    "matchup" : {
      "batter" : {
        "id" : 542248,
        "fullName" : "Bernardo Villar",
        "link" : "/api/v1/people/542248"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 501525,
        "fullName" : "Edwin Bernabel",
        "link" : "/api/v1/people/501525"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Men_On"
      }
    },
    "pitchIndex" : [ 1 ],
    "actionIndex" : [ 0 ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Hit By Pitch",
        "eventType" : "hit_by_pitch",
        "movementReason" : null,
        "runner" : {
          "id" : 542248,
          "fullName" : "Bernardo Villar",
          "link" : "/api/v1/people/542248"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 1
      },
      "credits" : [ ]
    } ],
    "playEvents" : [ {
      "details" : {
        "description" : "Pitcher Change: Edwin Bernabel replaces Gabirel Arias.",
        "event" : "Pitching Substitution",
        "awayScore" : 4,
        "homeScore" : 7,
        "isScoringPlay" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0,
        "outs" : 0
      },
      "index" : 0,
      "isPitch" : false,
      "type" : "action",
      "player" : {
        "id" : 501525,
        "link" : "/api/v1/people/501525"
      },
      "position" : {
        "code" : "1",
        "name" : "Pitcher",
        "type" : "Pitcher",
        "abbreviation" : "P"
      }
    }, {
      "details" : {
        "call" : {
          "code" : "H",
          "description" : "Ball - Hit by Pitch"
        },
        "description" : "Hit By Pitch",
        "code" : "H",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 87.55,
          "y" : 113.11
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0746-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 73
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Strikeout",
      "eventType" : "strikeout",
      "description" : "Roberto Reyes strikes out swinging.",
      "rbi" : 0,
      "awayScore" : 4,
      "homeScore" : 7
    },
    "about" : {
      "atBatIndex" : 74,
      "halfInning" : "top",
      "inning" : 9,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 14
    },
    "count" : {
      "balls" : 0,
      "strikes" : 3,
      "outs" : 1
    },
    "matchup" : {
      "batter" : {
        "id" : 542465,
        "fullName" : "Roberto Reyes",
        "link" : "/api/v1/people/542465"
      },
      "batSide" : {
        "code" : "L",
        "description" : "Left"
      },
      "pitcher" : {
        "id" : 501525,
        "fullName" : "Edwin Bernabel",
        "link" : "/api/v1/people/501525"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_LHB",
        "menOnBase" : "Men_On"
      }
    },
    "pitchIndex" : [ 0, 1, 2 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 1
      },
      "details" : {
        "event" : "Strikeout",
        "eventType" : "strikeout",
        "movementReason" : null,
        "runner" : {
          "id" : 542465,
          "fullName" : "Roberto Reyes",
          "link" : "/api/v1/people/542465"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 2
      },
      "credits" : [ {
        "player" : {
          "id" : 501517,
          "link" : "/api/v1/people/501517"
        },
        "position" : {
          "code" : "2",
          "name" : "Catcher",
          "type" : "Catcher",
          "abbreviation" : "C"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 1
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 87.55,
          "y" : 126.06
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0756-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 2
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 103.0,
          "y" : 120.88
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0756-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 3
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 90.99,
          "y" : 113.98
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0756-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 74
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Strikeout",
      "eventType" : "strikeout",
      "description" : "Marcos Martinez strikes out swinging.",
      "rbi" : 0,
      "awayScore" : 4,
      "homeScore" : 7
    },
    "about" : {
      "atBatIndex" : 75,
      "halfInning" : "top",
      "inning" : 9,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 14
    },
    "count" : {
      "balls" : 0,
      "strikes" : 3,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 516658,
        "fullName" : "Marcos Martinez",
        "link" : "/api/v1/people/516658"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 501525,
        "fullName" : "Edwin Bernabel",
        "link" : "/api/v1/people/501525"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Men_On"
      }
    },
    "pitchIndex" : [ 0, 1, 2 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 2
      },
      "details" : {
        "event" : "Strikeout",
        "eventType" : "strikeout",
        "movementReason" : null,
        "runner" : {
          "id" : 516658,
          "fullName" : "Marcos Martinez",
          "link" : "/api/v1/people/516658"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 2
      },
      "credits" : [ {
        "player" : {
          "id" : 501517,
          "link" : "/api/v1/people/501517"
        },
        "position" : {
          "code" : "2",
          "name" : "Catcher",
          "type" : "Catcher",
          "abbreviation" : "C"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 1
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 83.26,
          "y" : 118.29
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0766-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 2
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 86.7,
          "y" : 125.2
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0766-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 3
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 96.14,
          "y" : 120.88
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0766-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 75
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Walk",
      "eventType" : "walk",
      "description" : "Wader Perez walks.    Bernardo Villar to 2nd.",
      "rbi" : 0,
      "awayScore" : 4,
      "homeScore" : 7
    },
    "about" : {
      "atBatIndex" : 76,
      "halfInning" : "top",
      "inning" : 9,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 4,
      "strikes" : 0,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 542458,
        "fullName" : "Wader Perez",
        "link" : "/api/v1/people/542458"
      },
      "batSide" : {
        "code" : "L",
        "description" : "Left"
      },
      "pitcher" : {
        "id" : 501525,
        "fullName" : "Edwin Bernabel",
        "link" : "/api/v1/people/501525"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_LHB",
        "menOnBase" : "RISP"
      }
    },
    "pitchIndex" : [ 0, 1, 2, 3 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0, 1 ],
    "runners" : [ {
      "movement" : {
        "start" : "1B",
        "end" : "2B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Walk",
        "eventType" : "walk",
        "movementReason" : "r_adv_force",
        "runner" : {
          "id" : 542248,
          "fullName" : "Bernardo Villar",
          "link" : "/api/v1/people/542248"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 3
      }
    }, {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Walk",
        "eventType" : "walk",
        "movementReason" : null,
        "runner" : {
          "id" : 542458,
          "fullName" : "Wader Perez",
          "link" : "/api/v1/people/542458"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 3
      },
      "credits" : [ ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 126.18,
          "y" : 117.43
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0776-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 2,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 121.03,
          "y" : 125.2
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0776-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 3,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 119.31,
          "y" : 126.06
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0776-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 4,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 117.6,
          "y" : 114.84
        },
        "breaks" : { }
      },
      "index" : 3,
      "playId" : "02401986-0776-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 76
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Walk",
      "eventType" : "walk",
      "description" : "Alexander Castellano walks.",
      "rbi" : 0,
      "awayScore" : 4,
      "homeScore" : 7
    },
    "about" : {
      "atBatIndex" : 77,
      "halfInning" : "top",
      "inning" : 9,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 4,
      "strikes" : 0,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 542463,
        "fullName" : "Alex Castellano",
        "link" : "/api/v1/people/542463"
      },
      "batSide" : {
        "code" : "L",
        "description" : "Left"
      },
      "pitcher" : {
        "id" : 501525,
        "fullName" : "Edwin Bernabel",
        "link" : "/api/v1/people/501525"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_LHB",
        "menOnBase" : "Loaded"
      }
    },
    "pitchIndex" : [ 0, 2, 3, 4, 5 ],
    "actionIndex" : [ 1 ],
    "runnerIndex" : [ 2 ],
    "runners" : [ {
      "movement" : {
        "start" : "2B",
        "end" : "3B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Pickoff Error 2B",
        "eventType" : "pickoff_error_2b",
        "movementReason" : "r_adv_play",
        "runner" : {
          "id" : 542248,
          "fullName" : "Bernardo Villar",
          "link" : "/api/v1/people/542248"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 1
      },
      "credits" : [ {
        "player" : {
          "id" : 501517,
          "link" : "/api/v1/people/501517"
        },
        "position" : {
          "code" : "2",
          "name" : "Catcher",
          "type" : "Catcher",
          "abbreviation" : "C"
        },
        "credit" : "f_throwing_error"
      } ]
    }, {
      "movement" : {
        "start" : "1B",
        "end" : "2B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Pickoff Error 2B",
        "eventType" : "pickoff_error_2b",
        "movementReason" : "r_adv_play",
        "runner" : {
          "id" : 542458,
          "fullName" : "Wader Perez",
          "link" : "/api/v1/people/542458"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 1
      }
    }, {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Walk",
        "eventType" : "walk",
        "movementReason" : null,
        "runner" : {
          "id" : 542463,
          "fullName" : "Alex Castellano",
          "link" : "/api/v1/people/542463"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 5
      },
      "credits" : [ ]
    } ],
    "playEvents" : [ {
      "details" : {
        "description" : "Pickoff Attempt 2B",
        "code" : "2",
        "hasReview" : false,
        "fromCatcher" : true
      },
      "count" : { },
      "index" : 0,
      "isPitch" : false,
      "type" : "pickoff"
    }, {
      "details" : {
        "description" : "With Alexander Castellano batting, throwing error by Emmanuel Checo on the pickoff attempt, Bernardo Villar to 3rd.    Wader Perez to 2nd.",
        "event" : "Pickoff Error 2B",
        "eventType" : "pickoff_error_2b",
        "awayScore" : 4,
        "homeScore" : 7,
        "isScoringPlay" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0,
        "outs" : 2
      },
      "index" : 1,
      "isPitch" : false,
      "type" : "action",
      "player" : {
        "id" : 501517,
        "link" : "/api/v1/people/501517"
      }
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 121.03,
          "y" : 116.57
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0786-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 2,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 115.88,
          "y" : 107.07
        },
        "breaks" : { }
      },
      "index" : 3,
      "playId" : "02401986-0786-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 3,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 124.46,
          "y" : 114.84
        },
        "breaks" : { }
      },
      "index" : 4,
      "playId" : "02401986-0786-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 4,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 127.04,
          "y" : 123.47
        },
        "breaks" : { }
      },
      "index" : 5,
      "playId" : "02401986-0786-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 77
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Single",
      "eventType" : "single",
      "description" : "Juan B Cabrera singles on a line drive to left fielder Jose Trinidad.    Bernardo Villar scores.    Wader Perez scores.    Alexander Castellano to 2nd.",
      "rbi" : 2,
      "awayScore" : 6,
      "homeScore" : 7
    },
    "about" : {
      "atBatIndex" : 78,
      "halfInning" : "top",
      "inning" : 9,
      "isComplete" : true,
      "isScoringPlay" : true,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 90
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 516868,
        "fullName" : "Juan B Cabrera",
        "link" : "/api/v1/people/516868"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 469192,
        "fullName" : "Joel Martinez",
        "link" : "/api/v1/people/469192"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "RISP"
      }
    },
    "pitchIndex" : [ 1 ],
    "actionIndex" : [ 0 ],
    "runnerIndex" : [ 0, 1, 2, 3, 4 ],
    "runners" : [ {
      "movement" : {
        "start" : "3B",
        "end" : "score",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : "r_adv_force",
        "runner" : {
          "id" : 542248,
          "fullName" : "Bernardo Villar",
          "link" : "/api/v1/people/542248"
        },
        "responsiblePitcher" : {
          "id" : 501525,
          "link" : "/api/v1/people/501525"
        },
        "isScoringEvent" : true,
        "rbi" : true,
        "earned" : true,
        "teamUnearned" : false,
        "playIndex" : 1
      }
    }, {
      "movement" : {
        "start" : "2B",
        "end" : "3B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : "r_adv_force",
        "runner" : {
          "id" : 542458,
          "fullName" : "Wader Perez",
          "link" : "/api/v1/people/542458"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 1
      }
    }, {
      "movement" : {
        "start" : "3B",
        "end" : "score",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : "r_adv_play",
        "runner" : {
          "id" : 542458,
          "fullName" : "Wader Perez",
          "link" : "/api/v1/people/542458"
        },
        "responsiblePitcher" : {
          "id" : 501525,
          "link" : "/api/v1/people/501525"
        },
        "isScoringEvent" : true,
        "rbi" : true,
        "earned" : true,
        "teamUnearned" : false,
        "playIndex" : 1
      }
    }, {
      "movement" : {
        "start" : "1B",
        "end" : "2B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : "r_adv_force",
        "runner" : {
          "id" : 542463,
          "fullName" : "Alex Castellano",
          "link" : "/api/v1/people/542463"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 1
      }
    }, {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : null,
        "runner" : {
          "id" : 516868,
          "fullName" : "Juan B Cabrera",
          "link" : "/api/v1/people/516868"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 1
      },
      "credits" : [ {
        "player" : {
          "id" : 542339,
          "link" : "/api/v1/people/542339"
        },
        "position" : {
          "code" : "7",
          "name" : "Outfielder",
          "type" : "Outfielder",
          "abbreviation" : "LF"
        },
        "credit" : "f_fielded_ball"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "description" : "Pitcher Change: Joel Martinez replaces Edwin Bernabel.",
        "event" : "Pitching Substitution",
        "awayScore" : 4,
        "homeScore" : 7,
        "isScoringPlay" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0,
        "outs" : 2
      },
      "index" : 0,
      "isPitch" : false,
      "type" : "action",
      "player" : {
        "id" : 469192,
        "link" : "/api/v1/people/469192"
      },
      "position" : {
        "code" : "1",
        "name" : "Pitcher",
        "type" : "Pitcher",
        "abbreviation" : "P"
      }
    }, {
      "details" : {
        "call" : {
          "code" : "E",
          "description" : "Hit Into Play - Run(s)"
        },
        "description" : "In play, run(s)",
        "code" : "E",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 86.7,
          "y" : 120.02
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "line_drive",
        "hardness" : "medium",
        "location" : "7",
        "coordinates" : {
          "coordX" : 69.28,
          "coordY" : 119.48
        }
      },
      "index" : 1,
      "playId" : "02401986-0796-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 78
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Single",
      "eventType" : "single",
      "description" : "Audris Perez singles on a line drive to center fielder Miguel Alvarez.    Alexander Castellano scores.    Audris Perez to 2nd.  Juan B Cabrera advances to 3rd, on fielding error by center fielder Miguel Alvarez.",
      "rbi" : 1,
      "awayScore" : 7,
      "homeScore" : 7
    },
    "about" : {
      "atBatIndex" : 79,
      "halfInning" : "top",
      "inning" : 9,
      "isComplete" : true,
      "isScoringPlay" : true,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 70
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 542233,
        "fullName" : "Audry Perez",
        "link" : "/api/v1/people/542233"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 469192,
        "fullName" : "Joel Martinez",
        "link" : "/api/v1/people/469192"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "RISP"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0, 1, 2, 3, 4, 5 ],
    "runners" : [ {
      "movement" : {
        "start" : "2B",
        "end" : "3B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : "r_adv_force",
        "runner" : {
          "id" : 542463,
          "fullName" : "Alex Castellano",
          "link" : "/api/v1/people/542463"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      }
    }, {
      "movement" : {
        "start" : "3B",
        "end" : "score",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : "r_adv_play",
        "runner" : {
          "id" : 542463,
          "fullName" : "Alex Castellano",
          "link" : "/api/v1/people/542463"
        },
        "responsiblePitcher" : {
          "id" : 501525,
          "link" : "/api/v1/people/501525"
        },
        "isScoringEvent" : true,
        "rbi" : true,
        "earned" : true,
        "teamUnearned" : false,
        "playIndex" : 0
      }
    }, {
      "movement" : {
        "start" : "1B",
        "end" : "2B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : "r_adv_force",
        "runner" : {
          "id" : 516868,
          "fullName" : "Juan B Cabrera",
          "link" : "/api/v1/people/516868"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      }
    }, {
      "movement" : {
        "start" : "2B",
        "end" : "3B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Error",
        "eventType" : "error",
        "movementReason" : "r_adv_play",
        "runner" : {
          "id" : 516868,
          "fullName" : "Juan B Cabrera",
          "link" : "/api/v1/people/516868"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 516758,
          "link" : "/api/v1/people/516758"
        },
        "position" : {
          "code" : "8",
          "name" : "Outfielder",
          "type" : "Outfielder",
          "abbreviation" : "CF"
        },
        "credit" : "f_fielding_error"
      } ]
    }, {
      "movement" : {
        "start" : "1B",
        "end" : "2B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Error",
        "eventType" : "error",
        "movementReason" : "r_adv_play",
        "runner" : {
          "id" : 542233,
          "fullName" : "Audry Perez",
          "link" : "/api/v1/people/542233"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      }
    }, {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : null,
        "runner" : {
          "id" : 542233,
          "fullName" : "Audry Perez",
          "link" : "/api/v1/people/542233"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 516758,
          "link" : "/api/v1/people/516758"
        },
        "position" : {
          "code" : "8",
          "name" : "Outfielder",
          "type" : "Outfielder",
          "abbreviation" : "CF"
        },
        "credit" : "f_fielded_ball"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "E",
          "description" : "Hit Into Play - Run(s)"
        },
        "description" : "In play, run(s)",
        "code" : "E",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 92.7,
          "y" : 119.16
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "line_drive",
        "hardness" : "medium",
        "location" : "8",
        "coordinates" : {
          "coordX" : 56.22,
          "coordY" : 121.49
        }
      },
      "index" : 0,
      "playId" : "02401986-0806-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 79
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Groundout",
      "eventType" : "field_out",
      "description" : "Luis Pimentel grounds out, second baseman Carlos Valenzuela to first baseman Geancarlo Mendez.",
      "rbi" : 0,
      "awayScore" : 7,
      "homeScore" : 7
    },
    "about" : {
      "atBatIndex" : 80,
      "halfInning" : "top",
      "inning" : 9,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 3
    },
    "matchup" : {
      "batter" : {
        "id" : 540938,
        "fullName" : "Luis Pimentel",
        "link" : "/api/v1/people/540938"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 469192,
        "fullName" : "Joel Martinez",
        "link" : "/api/v1/people/469192"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 3
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : null,
        "runner" : {
          "id" : 540938,
          "fullName" : "Luis Pimentel",
          "link" : "/api/v1/people/540938"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 542477,
          "link" : "/api/v1/people/542477"
        },
        "position" : {
          "code" : "4",
          "name" : "Second Base",
          "type" : "Infielder",
          "abbreviation" : "2B"
        },
        "credit" : "f_assist"
      }, {
        "player" : {
          "id" : 542479,
          "link" : "/api/v1/people/542479"
        },
        "position" : {
          "code" : "3",
          "name" : "First Base",
          "type" : "Infielder",
          "abbreviation" : "1B"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 94.42,
          "y" : 116.57
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "ground_ball",
        "hardness" : "medium",
        "location" : "4",
        "coordinates" : {
          "coordX" : 127.51,
          "coordY" : 168.67
        }
      },
      "index" : 0,
      "playId" : "02401986-0816-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 80
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Walk",
      "eventType" : "walk",
      "description" : "Geancarlo Mendez walks.",
      "rbi" : 0,
      "awayScore" : 7,
      "homeScore" : 7
    },
    "about" : {
      "atBatIndex" : 81,
      "halfInning" : "bottom",
      "inning" : 9,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 4,
      "strikes" : 0,
      "outs" : 0
    },
    "matchup" : {
      "batter" : {
        "id" : 542479,
        "fullName" : "Geancarlo Mendez",
        "link" : "/api/v1/people/542479"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542524,
        "fullName" : "Jose Almarante",
        "link" : "/api/v1/people/542524"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Men_On"
      }
    },
    "pitchIndex" : [ 1, 2, 3, 4 ],
    "actionIndex" : [ 0 ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Walk",
        "eventType" : "walk",
        "movementReason" : null,
        "runner" : {
          "id" : 542479,
          "fullName" : "Geancarlo Mendez",
          "link" : "/api/v1/people/542479"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 4
      },
      "credits" : [ ]
    } ],
    "playEvents" : [ {
      "details" : {
        "description" : "Pitcher Change: Eduard Estalis replaces Amaury Castillo.",
        "event" : "Pitching Substitution",
        "awayScore" : 7,
        "homeScore" : 7,
        "isScoringPlay" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0,
        "outs" : 0
      },
      "index" : 0,
      "isPitch" : false,
      "type" : "action",
      "player" : {
        "id" : 542524,
        "link" : "/api/v1/people/542524"
      },
      "position" : {
        "code" : "1",
        "name" : "Pitcher",
        "type" : "Pitcher",
        "abbreviation" : "P"
      }
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 52.36,
          "y" : 108.8
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0826-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 2,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 51.5,
          "y" : 108.8
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0826-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 3,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 47.21,
          "y" : 103.61
        },
        "breaks" : { }
      },
      "index" : 3,
      "playId" : "02401986-0826-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 4,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 42.06,
          "y" : 96.71
        },
        "breaks" : { }
      },
      "index" : 4,
      "playId" : "02401986-0826-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 81
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Sac Bunt",
      "eventType" : "sac_bunt",
      "description" : "Emmanuel Checo out on a sacrifice bunt, pitcher Eduard Estalis to second baseman Wader Perez.    Geancarlo Mendez to 2nd.",
      "rbi" : 0,
      "awayScore" : 7,
      "homeScore" : 7
    },
    "about" : {
      "atBatIndex" : 82,
      "halfInning" : "bottom",
      "inning" : 9,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 4
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 1
    },
    "matchup" : {
      "batter" : {
        "id" : 501517,
        "fullName" : "Emmanuel Checo",
        "link" : "/api/v1/people/501517"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542524,
        "fullName" : "Jose Almarante",
        "link" : "/api/v1/people/542524"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "RISP"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0, 1 ],
    "runners" : [ {
      "movement" : {
        "start" : "1B",
        "end" : "2B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Sac Bunt",
        "eventType" : "sac_bunt",
        "movementReason" : "r_adv_play",
        "runner" : {
          "id" : 542479,
          "fullName" : "Geancarlo Mendez",
          "link" : "/api/v1/people/542479"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      }
    }, {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 1
      },
      "details" : {
        "event" : "Sac Bunt",
        "eventType" : "sac_bunt",
        "movementReason" : null,
        "runner" : {
          "id" : 501517,
          "fullName" : "Emmanuel Checo",
          "link" : "/api/v1/people/501517"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 542524,
          "link" : "/api/v1/people/542524"
        },
        "position" : {
          "code" : "1",
          "name" : "Pitcher",
          "type" : "Pitcher",
          "abbreviation" : "P"
        },
        "credit" : "f_assist"
      }, {
        "player" : {
          "id" : 542458,
          "link" : "/api/v1/people/542458"
        },
        "position" : {
          "code" : "4",
          "name" : "Second Base",
          "type" : "Infielder",
          "abbreviation" : "2B"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 91.85,
          "y" : 118.29
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "bunt_grounder",
        "hardness" : "medium",
        "location" : "1",
        "coordinates" : {
          "coordX" : 120.48,
          "coordY" : 198.8
        }
      },
      "index" : 0,
      "playId" : "02401986-0836-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 82
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Intent Walk",
      "eventType" : "intent_walk",
      "description" : "Eduard Estalis intentionally walks Luis Paulino.",
      "rbi" : 0,
      "awayScore" : 7,
      "homeScore" : 7
    },
    "about" : {
      "atBatIndex" : 83,
      "halfInning" : "bottom",
      "inning" : 9,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 4,
      "strikes" : 0,
      "outs" : 1
    },
    "matchup" : {
      "batter" : {
        "id" : 516760,
        "fullName" : "Luis Paulino",
        "link" : "/api/v1/people/516760"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542524,
        "fullName" : "Jose Almarante",
        "link" : "/api/v1/people/542524"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "RISP"
      }
    },
    "pitchIndex" : [ 0, 1, 2, 3 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Intent Walk",
        "eventType" : "intent_walk",
        "movementReason" : null,
        "runner" : {
          "id" : 516760,
          "fullName" : "Luis Paulino",
          "link" : "/api/v1/people/516760"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 3
      },
      "credits" : [ ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "I",
          "description" : "Ball - Intentional"
        },
        "description" : "Intent Ball",
        "code" : "I",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 54.08,
          "y" : 101.89
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0846-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "I",
          "description" : "Ball - Intentional"
        },
        "description" : "Intent Ball",
        "code" : "I",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 2,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 49.79,
          "y" : 106.2
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0846-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "I",
          "description" : "Ball - Intentional"
        },
        "description" : "Intent Ball",
        "code" : "I",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 3,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 49.79,
          "y" : 104.48
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0846-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "I",
          "description" : "Ball - Intentional"
        },
        "description" : "Intent Ball",
        "code" : "I",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 4,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 43.78,
          "y" : 101.02
        },
        "breaks" : { }
      },
      "index" : 3,
      "playId" : "02401986-0846-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 83
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Walk",
      "eventType" : "walk",
      "description" : "Cesar Tejada walks.    Geancarlo Mendez to 3rd.    Luis Paulino to 2nd.",
      "rbi" : 0,
      "awayScore" : 7,
      "homeScore" : 7
    },
    "about" : {
      "atBatIndex" : 84,
      "halfInning" : "bottom",
      "inning" : 9,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 4,
      "strikes" : 0,
      "outs" : 1
    },
    "matchup" : {
      "batter" : {
        "id" : 516686,
        "fullName" : "Cesar Tejada",
        "link" : "/api/v1/people/516686"
      },
      "batSide" : {
        "code" : "L",
        "description" : "Left"
      },
      "pitcher" : {
        "id" : 542524,
        "fullName" : "Jose Almarante",
        "link" : "/api/v1/people/542524"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_LHB",
        "menOnBase" : "Loaded"
      }
    },
    "pitchIndex" : [ 1, 2, 3, 4 ],
    "actionIndex" : [ 0 ],
    "runnerIndex" : [ 0, 1, 2 ],
    "runners" : [ {
      "movement" : {
        "start" : "2B",
        "end" : "3B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Walk",
        "eventType" : "walk",
        "movementReason" : "r_adv_force",
        "runner" : {
          "id" : 542479,
          "fullName" : "Geancarlo Mendez",
          "link" : "/api/v1/people/542479"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 4
      }
    }, {
      "movement" : {
        "start" : "1B",
        "end" : "2B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Walk",
        "eventType" : "walk",
        "movementReason" : "r_adv_force",
        "runner" : {
          "id" : 516760,
          "fullName" : "Luis Paulino",
          "link" : "/api/v1/people/516760"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 4
      }
    }, {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Walk",
        "eventType" : "walk",
        "movementReason" : null,
        "runner" : {
          "id" : 516686,
          "fullName" : "Cesar Tejada",
          "link" : "/api/v1/people/516686"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 4
      },
      "credits" : [ ]
    } ],
    "playEvents" : [ {
      "details" : {
        "description" : "Offensive Substitution: Pinch hitter Cesar Tejada replaces Jose Trinidad.",
        "event" : "Offensive Substitution",
        "eventType" : "offensive_substitution",
        "awayScore" : 7,
        "homeScore" : 7,
        "isScoringPlay" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0,
        "outs" : 1
      },
      "index" : 0,
      "isPitch" : false,
      "type" : "action",
      "player" : {
        "id" : 516686,
        "link" : "/api/v1/people/516686"
      },
      "battingOrder" : "801"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 124.46,
          "y" : 117.43
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0856-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 2,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 127.04,
          "y" : 120.88
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0856-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 3,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 121.89,
          "y" : 132.11
        },
        "breaks" : { }
      },
      "index" : 3,
      "playId" : "02401986-0856-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 4,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 123.61,
          "y" : 112.25
        },
        "breaks" : { }
      },
      "index" : 4,
      "playId" : "02401986-0856-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 84
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Strikeout",
      "eventType" : "strikeout",
      "description" : "Eduard Martinez called out on strikes.",
      "rbi" : 0,
      "awayScore" : 7,
      "homeScore" : 7
    },
    "about" : {
      "atBatIndex" : 85,
      "halfInning" : "bottom",
      "inning" : 9,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 14
    },
    "count" : {
      "balls" : 0,
      "strikes" : 3,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 499960,
        "fullName" : "Eduard Martinez",
        "link" : "/api/v1/people/499960"
      },
      "batSide" : {
        "code" : "L",
        "description" : "Left"
      },
      "pitcher" : {
        "id" : 542524,
        "fullName" : "Jose Almarante",
        "link" : "/api/v1/people/542524"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_LHB",
        "menOnBase" : "Loaded"
      }
    },
    "pitchIndex" : [ 0, 1, 2 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 2
      },
      "details" : {
        "event" : "Strikeout",
        "eventType" : "strikeout",
        "movementReason" : null,
        "runner" : {
          "id" : 499960,
          "fullName" : "Eduard Martinez",
          "link" : "/api/v1/people/499960"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 2
      },
      "credits" : [ {
        "player" : {
          "id" : 542233,
          "link" : "/api/v1/people/542233"
        },
        "position" : {
          "code" : "2",
          "name" : "Catcher",
          "type" : "Catcher",
          "abbreviation" : "C"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "C",
          "description" : "Strike - Called"
        },
        "description" : "Called Strike",
        "code" : "C",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 1
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 87.55,
          "y" : 118.29
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0866-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "C",
          "description" : "Strike - Called"
        },
        "description" : "Called Strike",
        "code" : "C",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 2
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 94.42,
          "y" : 120.88
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0866-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "C",
          "description" : "Strike - Called"
        },
        "description" : "Called Strike",
        "code" : "C",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 3
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 105.58,
          "y" : 120.88
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0866-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 85
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Strikeout",
      "eventType" : "strikeout",
      "description" : "Jonathan Villan strikes out swinging.",
      "rbi" : 0,
      "awayScore" : 7,
      "homeScore" : 7
    },
    "about" : {
      "atBatIndex" : 86,
      "halfInning" : "bottom",
      "inning" : 9,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 14
    },
    "count" : {
      "balls" : 0,
      "strikes" : 3,
      "outs" : 3
    },
    "matchup" : {
      "batter" : {
        "id" : 542340,
        "fullName" : "Jonathan Villar",
        "link" : "/api/v1/people/542340"
      },
      "batSide" : {
        "code" : "L",
        "description" : "Left"
      },
      "pitcher" : {
        "id" : 542524,
        "fullName" : "Jose Almarante",
        "link" : "/api/v1/people/542524"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_LHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0, 1, 2 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 3
      },
      "details" : {
        "event" : "Strikeout",
        "eventType" : "strikeout",
        "movementReason" : null,
        "runner" : {
          "id" : 542340,
          "fullName" : "Jonathan Villar",
          "link" : "/api/v1/people/542340"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 2
      },
      "credits" : [ {
        "player" : {
          "id" : 542233,
          "link" : "/api/v1/people/542233"
        },
        "position" : {
          "code" : "2",
          "name" : "Catcher",
          "type" : "Catcher",
          "abbreviation" : "C"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 1
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 90.99,
          "y" : 113.98
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0876-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 2
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 90.99,
          "y" : 116.57
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0876-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 3
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 90.13,
          "y" : 113.98
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0876-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 86
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Flyout",
      "eventType" : "field_out",
      "description" : "Santo Sandoval flies out to left fielder Cesar Tejada.",
      "rbi" : 0,
      "awayScore" : 7,
      "homeScore" : 7
    },
    "about" : {
      "atBatIndex" : 87,
      "halfInning" : "top",
      "inning" : 10,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 1
    },
    "matchup" : {
      "batter" : {
        "id" : 542468,
        "fullName" : "Santo Sandoval",
        "link" : "/api/v1/people/542468"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 545002,
        "fullName" : "Juan Sepulveda",
        "link" : "/api/v1/people/545002"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 2 ],
    "actionIndex" : [ 0, 1 ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 1
      },
      "details" : {
        "event" : "Field Out",
        "eventType" : "field_out",
        "movementReason" : null,
        "runner" : {
          "id" : 542468,
          "fullName" : "Santo Sandoval",
          "link" : "/api/v1/people/542468"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 2
      },
      "credits" : [ {
        "player" : {
          "id" : 516686,
          "link" : "/api/v1/people/516686"
        },
        "position" : {
          "code" : "7",
          "name" : "Outfielder",
          "type" : "Outfielder",
          "abbreviation" : "LF"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "description" : "Cesar Tejada remains in the game as the left fielder.",
        "event" : "Defensive Switch",
        "eventType" : "defensive_switch",
        "awayScore" : 7,
        "homeScore" : 7,
        "isScoringPlay" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0,
        "outs" : 0
      },
      "index" : 0,
      "isPitch" : false,
      "type" : "action",
      "player" : {
        "id" : 516686,
        "link" : "/api/v1/people/516686"
      },
      "battingOrder" : "801"
    }, {
      "details" : {
        "description" : "Pitcher Change: Juan Sepulveda replaces Joel Martinez.",
        "event" : "Pitching Substitution",
        "awayScore" : 7,
        "homeScore" : 7,
        "isScoringPlay" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0,
        "outs" : 0
      },
      "index" : 1,
      "isPitch" : false,
      "type" : "action",
      "player" : {
        "id" : 545002,
        "link" : "/api/v1/people/545002"
      },
      "position" : {
        "code" : "1",
        "name" : "Pitcher",
        "type" : "Pitcher",
        "abbreviation" : "P"
      }
    }, {
      "details" : {
        "call" : {
          "code" : "X",
          "description" : "Hit Into Play - Out(s)"
        },
        "description" : "In play, out(s)",
        "code" : "X",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 92.7,
          "y" : 127.79
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "fly_ball",
        "hardness" : "medium",
        "location" : "7",
        "coordinates" : {
          "coordX" : 53.21,
          "coordY" : 115.46
        }
      },
      "index" : 2,
      "playId" : "02401986-0886-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 87
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Strikeout",
      "eventType" : "strikeout",
      "description" : "Bernardo Villar strikes out swinging.",
      "rbi" : 0,
      "awayScore" : 7,
      "homeScore" : 7
    },
    "about" : {
      "atBatIndex" : 88,
      "halfInning" : "top",
      "inning" : 10,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 14
    },
    "count" : {
      "balls" : 0,
      "strikes" : 3,
      "outs" : 2
    },
    "matchup" : {
      "batter" : {
        "id" : 542248,
        "fullName" : "Bernardo Villar",
        "link" : "/api/v1/people/542248"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 545002,
        "fullName" : "Juan Sepulveda",
        "link" : "/api/v1/people/545002"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0, 1, 2 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 2
      },
      "details" : {
        "event" : "Strikeout",
        "eventType" : "strikeout",
        "movementReason" : null,
        "runner" : {
          "id" : 542248,
          "fullName" : "Bernardo Villar",
          "link" : "/api/v1/people/542248"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 2
      },
      "credits" : [ {
        "player" : {
          "id" : 501517,
          "link" : "/api/v1/people/501517"
        },
        "position" : {
          "code" : "2",
          "name" : "Catcher",
          "type" : "Catcher",
          "abbreviation" : "C"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 1
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 100.43,
          "y" : 119.16
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0896-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 2
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 87.55,
          "y" : 121.75
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0896-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "S",
          "description" : "Strike - Swinging"
        },
        "description" : "Swinging Strike",
        "code" : "S",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 3
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 90.99,
          "y" : 120.88
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0896-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 88
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Strikeout",
      "eventType" : "strikeout",
      "description" : "Roberto Reyes called out on strikes.",
      "rbi" : 0,
      "awayScore" : 7,
      "homeScore" : 7
    },
    "about" : {
      "atBatIndex" : 89,
      "halfInning" : "top",
      "inning" : 10,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : true,
      "captivatingIndex" : 14
    },
    "count" : {
      "balls" : 0,
      "strikes" : 3,
      "outs" : 3
    },
    "matchup" : {
      "batter" : {
        "id" : 542465,
        "fullName" : "Roberto Reyes",
        "link" : "/api/v1/people/542465"
      },
      "batSide" : {
        "code" : "L",
        "description" : "Left"
      },
      "pitcher" : {
        "id" : 545002,
        "fullName" : "Juan Sepulveda",
        "link" : "/api/v1/people/545002"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_LHB",
        "menOnBase" : "Empty"
      }
    },
    "pitchIndex" : [ 0, 1, 2 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : null,
        "outBase" : "1B",
        "isOut" : true,
        "outNumber" : 3
      },
      "details" : {
        "event" : "Strikeout",
        "eventType" : "strikeout",
        "movementReason" : null,
        "runner" : {
          "id" : 542465,
          "fullName" : "Roberto Reyes",
          "link" : "/api/v1/people/542465"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 2
      },
      "credits" : [ {
        "player" : {
          "id" : 501517,
          "link" : "/api/v1/people/501517"
        },
        "position" : {
          "code" : "2",
          "name" : "Catcher",
          "type" : "Catcher",
          "abbreviation" : "C"
        },
        "credit" : "f_putout"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "C",
          "description" : "Strike - Called"
        },
        "description" : "Called Strike",
        "code" : "C",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 1
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 95.28,
          "y" : 130.38
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0906-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "C",
          "description" : "Strike - Called"
        },
        "description" : "Called Strike",
        "code" : "C",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 2
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 86.7,
          "y" : 107.93
        },
        "breaks" : { }
      },
      "index" : 1,
      "playId" : "02401986-0906-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "C",
          "description" : "Strike - Called"
        },
        "description" : "Called Strike",
        "code" : "C",
        "ballColor" : "rgba(170, 21, 11, 1.0)",
        "isInPlay" : false,
        "isStrike" : true,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 3
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 86.7,
          "y" : 122.61
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0906-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 89
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Field Error",
      "eventType" : "field_error",
      "description" : "Carlos Valenzuela reaches on fielding error by pitcher Eduard Estalis.",
      "rbi" : 0,
      "awayScore" : 7,
      "homeScore" : 7
    },
    "about" : {
      "atBatIndex" : 90,
      "halfInning" : "bottom",
      "inning" : 10,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 0
    },
    "matchup" : {
      "batter" : {
        "id" : 542477,
        "fullName" : "Carlos Valenzuela",
        "link" : "/api/v1/people/542477"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542524,
        "fullName" : "Jose Almarante",
        "link" : "/api/v1/people/542524"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Men_On"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0 ],
    "runners" : [ {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Field Error",
        "eventType" : "field_error",
        "movementReason" : "r_adv_play",
        "runner" : {
          "id" : 542477,
          "fullName" : "Carlos Valenzuela",
          "link" : "/api/v1/people/542477"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 542524,
          "link" : "/api/v1/people/542524"
        },
        "position" : {
          "code" : "1",
          "name" : "Pitcher",
          "type" : "Pitcher",
          "abbreviation" : "P"
        },
        "credit" : "f_fielding_error"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "D",
          "description" : "Hit Into Play - No Out(s)"
        },
        "description" : "In play, no out",
        "code" : "D",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 97.0,
          "y" : 120.02
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "ground_ball",
        "hardness" : "medium",
        "location" : "1",
        "coordinates" : {
          "coordX" : 127.51,
          "coordY" : 189.76
        }
      },
      "index" : 0,
      "playId" : "02401986-0916-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 90
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Single",
      "eventType" : "single",
      "description" : "Miguel Alvarez singles on a line drive to left fielder Santo Sandoval.    Carlos Valenzuela to 3rd.",
      "rbi" : 0,
      "awayScore" : 7,
      "homeScore" : 7
    },
    "about" : {
      "atBatIndex" : 91,
      "halfInning" : "bottom",
      "inning" : 10,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 33
    },
    "count" : {
      "balls" : 1,
      "strikes" : 0,
      "outs" : 0
    },
    "matchup" : {
      "batter" : {
        "id" : 516758,
        "fullName" : "Miguel Alvarez",
        "link" : "/api/v1/people/516758"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542524,
        "fullName" : "Jose Almarante",
        "link" : "/api/v1/people/542524"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "RISP"
      }
    },
    "pitchIndex" : [ 0, 2 ],
    "actionIndex" : [ 1 ],
    "runnerIndex" : [ 1, 2 ],
    "runners" : [ {
      "movement" : {
        "start" : "1B",
        "end" : "2B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Stolen Base 2B",
        "eventType" : "stolen_base_2b",
        "movementReason" : "r_stolen_base_2b",
        "runner" : {
          "id" : 542477,
          "fullName" : "Carlos Valenzuela",
          "link" : "/api/v1/people/542477"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 1
      },
      "credits" : [ ]
    }, {
      "movement" : {
        "start" : "2B",
        "end" : "3B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : "r_adv_play",
        "runner" : {
          "id" : 542477,
          "fullName" : "Carlos Valenzuela",
          "link" : "/api/v1/people/542477"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 2
      }
    }, {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : null,
        "runner" : {
          "id" : 516758,
          "fullName" : "Miguel Alvarez",
          "link" : "/api/v1/people/516758"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 2
      },
      "credits" : [ {
        "player" : {
          "id" : 542468,
          "link" : "/api/v1/people/542468"
        },
        "position" : {
          "code" : "7",
          "name" : "Outfielder",
          "type" : "Outfielder",
          "abbreviation" : "LF"
        },
        "credit" : "f_fielded_ball"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 44.64,
          "y" : 119.16
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0926-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "description" : "With Miguel Alvarez batting, Carlos Valenzuela steals (10) 2nd base.",
        "event" : "Stolen Base 2B",
        "eventType" : "stolen_base_2b",
        "awayScore" : 7,
        "homeScore" : 7,
        "isScoringPlay" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0,
        "outs" : 0
      },
      "index" : 1,
      "isPitch" : false,
      "type" : "action",
      "player" : {
        "id" : 542477,
        "link" : "/api/v1/people/542477"
      }
    }, {
      "details" : {
        "call" : {
          "code" : "D",
          "description" : "Hit Into Play - No Out(s)"
        },
        "description" : "In play, no out",
        "code" : "D",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 94.42,
          "y" : 120.02
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "line_drive",
        "hardness" : "medium",
        "location" : "7",
        "coordinates" : {
          "coordX" : 76.31,
          "coordY" : 116.47
        }
      },
      "index" : 2,
      "playId" : "02401986-0926-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 91
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Intent Walk",
      "eventType" : "intent_walk",
      "description" : "Eduard Estalis intentionally walks Pedro Aguilar.",
      "rbi" : 0,
      "awayScore" : 7,
      "homeScore" : 7
    },
    "about" : {
      "atBatIndex" : 92,
      "halfInning" : "bottom",
      "inning" : 10,
      "isComplete" : true,
      "isScoringPlay" : false,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 0
    },
    "count" : {
      "balls" : 4,
      "strikes" : 0,
      "outs" : 0
    },
    "matchup" : {
      "batter" : {
        "id" : 516729,
        "fullName" : "Pedro Aguilar",
        "link" : "/api/v1/people/516729"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542524,
        "fullName" : "Jose Almarante",
        "link" : "/api/v1/people/542524"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Loaded"
      }
    },
    "pitchIndex" : [ 0, 2, 3, 4 ],
    "actionIndex" : [ 1 ],
    "runnerIndex" : [ 1 ],
    "runners" : [ {
      "movement" : {
        "start" : "1B",
        "end" : "2B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Stolen Base 2B",
        "eventType" : "stolen_base_2b",
        "movementReason" : "r_stolen_base_2b",
        "runner" : {
          "id" : 516758,
          "fullName" : "Miguel Alvarez",
          "link" : "/api/v1/people/516758"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 1
      },
      "credits" : [ ]
    }, {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Intent Walk",
        "eventType" : "intent_walk",
        "movementReason" : null,
        "runner" : {
          "id" : 516729,
          "fullName" : "Pedro Aguilar",
          "link" : "/api/v1/people/516729"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 4
      },
      "credits" : [ ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "B",
          "description" : "Ball - Called"
        },
        "description" : "Ball",
        "code" : "B",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 57.51,
          "y" : 120.88
        },
        "breaks" : { }
      },
      "index" : 0,
      "playId" : "02401986-0936-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "description" : "With Pedro Aguilar batting, Miguel Alvarez steals (19) 2nd base.",
        "event" : "Stolen Base 2B",
        "eventType" : "stolen_base_2b",
        "awayScore" : 7,
        "homeScore" : 7,
        "isScoringPlay" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 1,
        "strikes" : 0,
        "outs" : 0
      },
      "index" : 1,
      "isPitch" : false,
      "type" : "action",
      "player" : {
        "id" : 516758,
        "link" : "/api/v1/people/516758"
      }
    }, {
      "details" : {
        "call" : {
          "code" : "I",
          "description" : "Ball - Intentional"
        },
        "description" : "Intent Ball",
        "code" : "I",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 2,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 47.21,
          "y" : 115.7
        },
        "breaks" : { }
      },
      "index" : 2,
      "playId" : "02401986-0936-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "I",
          "description" : "Ball - Intentional"
        },
        "description" : "Intent Ball",
        "code" : "I",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 3,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 61.8,
          "y" : 112.25
        },
        "breaks" : { }
      },
      "index" : 3,
      "playId" : "02401986-0936-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    }, {
      "details" : {
        "call" : {
          "code" : "I",
          "description" : "Ball - Intentional"
        },
        "description" : "Intent Ball",
        "code" : "I",
        "ballColor" : "rgba(39, 161, 39, 1.0)",
        "isInPlay" : false,
        "isStrike" : false,
        "isBall" : true,
        "hasReview" : false
      },
      "count" : {
        "balls" : 4,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 48.93,
          "y" : 100.16
        },
        "breaks" : { }
      },
      "index" : 4,
      "playId" : "02401986-0936-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 92
  }, {
    "result" : {
      "type" : "atBat",
      "event" : "Single",
      "eventType" : "single",
      "description" : "Geancarlo Mendez singles on a line drive to left fielder Santo Sandoval.    Carlos Valenzuela scores.    Miguel Alvarez to 3rd.    Pedro Aguilar to 2nd.",
      "rbi" : 1,
      "awayScore" : 7,
      "homeScore" : 8
    },
    "about" : {
      "atBatIndex" : 93,
      "halfInning" : "bottom",
      "inning" : 10,
      "isComplete" : true,
      "isScoringPlay" : true,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 60
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 0
    },
    "matchup" : {
      "batter" : {
        "id" : 542479,
        "fullName" : "Geancarlo Mendez",
        "link" : "/api/v1/people/542479"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542524,
        "fullName" : "Jose Almarante",
        "link" : "/api/v1/people/542524"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Loaded"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0, 1, 2, 3 ],
    "runners" : [ {
      "movement" : {
        "start" : "3B",
        "end" : "score",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : "r_adv_force",
        "runner" : {
          "id" : 542477,
          "fullName" : "Carlos Valenzuela",
          "link" : "/api/v1/people/542477"
        },
        "responsiblePitcher" : {
          "id" : 542524,
          "link" : "/api/v1/people/542524"
        },
        "isScoringEvent" : true,
        "rbi" : true,
        "earned" : false,
        "teamUnearned" : true,
        "playIndex" : 0
      }
    }, {
      "movement" : {
        "start" : "2B",
        "end" : "3B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : "r_adv_force",
        "runner" : {
          "id" : 516758,
          "fullName" : "Miguel Alvarez",
          "link" : "/api/v1/people/516758"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      }
    }, {
      "movement" : {
        "start" : "1B",
        "end" : "2B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : "r_adv_force",
        "runner" : {
          "id" : 516729,
          "fullName" : "Pedro Aguilar",
          "link" : "/api/v1/people/516729"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      }
    }, {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : null,
        "runner" : {
          "id" : 542479,
          "fullName" : "Geancarlo Mendez",
          "link" : "/api/v1/people/542479"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 542468,
          "link" : "/api/v1/people/542468"
        },
        "position" : {
          "code" : "7",
          "name" : "Outfielder",
          "type" : "Outfielder",
          "abbreviation" : "LF"
        },
        "credit" : "f_fielded_ball"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "E",
          "description" : "Hit Into Play - Run(s)"
        },
        "description" : "In play, run(s)",
        "code" : "E",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 88.41,
          "y" : 114.84
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "line_drive",
        "hardness" : "medium",
        "location" : "7",
        "coordinates" : {
          "coordX" : 63.25,
          "coordY" : 117.47
        }
      },
      "index" : 0,
      "playId" : "02401986-0946-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 93
  } ],
  "currentPlay" : {
    "result" : {
      "type" : "atBat",
      "event" : "Single",
      "eventType" : "single",
      "description" : "Geancarlo Mendez singles on a line drive to left fielder Santo Sandoval.    Carlos Valenzuela scores.    Miguel Alvarez to 3rd.    Pedro Aguilar to 2nd.",
      "rbi" : 1,
      "awayScore" : 7,
      "homeScore" : 8
    },
    "about" : {
      "atBatIndex" : 93,
      "halfInning" : "bottom",
      "inning" : 10,
      "isComplete" : true,
      "isScoringPlay" : true,
      "hasReview" : false,
      "hasOut" : false,
      "captivatingIndex" : 60
    },
    "count" : {
      "balls" : 0,
      "strikes" : 0,
      "outs" : 0
    },
    "matchup" : {
      "batter" : {
        "id" : 542479,
        "fullName" : "Geancarlo Mendez",
        "link" : "/api/v1/people/542479"
      },
      "batSide" : {
        "code" : "R",
        "description" : "Right"
      },
      "pitcher" : {
        "id" : 542524,
        "fullName" : "Jose Almarante",
        "link" : "/api/v1/people/542524"
      },
      "pitchHand" : {
        "code" : "R",
        "description" : "Right"
      },
      "batterHotColdZones" : [ ],
      "pitcherHotColdZones" : [ ],
      "splits" : {
        "batter" : "vs_RHP",
        "pitcher" : "vs_RHB",
        "menOnBase" : "Loaded"
      }
    },
    "pitchIndex" : [ 0 ],
    "actionIndex" : [ ],
    "runnerIndex" : [ 0, 1, 2, 3 ],
    "runners" : [ {
      "movement" : {
        "start" : "3B",
        "end" : "score",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : "r_adv_force",
        "runner" : {
          "id" : 542477,
          "fullName" : "Carlos Valenzuela",
          "link" : "/api/v1/people/542477"
        },
        "responsiblePitcher" : {
          "id" : 542524,
          "link" : "/api/v1/people/542524"
        },
        "isScoringEvent" : true,
        "rbi" : true,
        "earned" : false,
        "teamUnearned" : true,
        "playIndex" : 0
      }
    }, {
      "movement" : {
        "start" : "2B",
        "end" : "3B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : "r_adv_force",
        "runner" : {
          "id" : 516758,
          "fullName" : "Miguel Alvarez",
          "link" : "/api/v1/people/516758"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      }
    }, {
      "movement" : {
        "start" : "1B",
        "end" : "2B",
        "outBase" : null,
        "isOut" : false,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : "r_adv_force",
        "runner" : {
          "id" : 516729,
          "fullName" : "Pedro Aguilar",
          "link" : "/api/v1/people/516729"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      }
    }, {
      "movement" : {
        "start" : null,
        "end" : "1B",
        "outBase" : null,
        "isOut" : null,
        "outNumber" : null
      },
      "details" : {
        "event" : "Single",
        "eventType" : "single",
        "movementReason" : null,
        "runner" : {
          "id" : 542479,
          "fullName" : "Geancarlo Mendez",
          "link" : "/api/v1/people/542479"
        },
        "responsiblePitcher" : null,
        "isScoringEvent" : false,
        "rbi" : false,
        "earned" : false,
        "teamUnearned" : false,
        "playIndex" : 0
      },
      "credits" : [ {
        "player" : {
          "id" : 542468,
          "link" : "/api/v1/people/542468"
        },
        "position" : {
          "code" : "7",
          "name" : "Outfielder",
          "type" : "Outfielder",
          "abbreviation" : "LF"
        },
        "credit" : "f_fielded_ball"
      } ]
    } ],
    "playEvents" : [ {
      "details" : {
        "call" : {
          "code" : "E",
          "description" : "Hit Into Play - Run(s)"
        },
        "description" : "In play, run(s)",
        "code" : "E",
        "ballColor" : "rgba(26, 86, 190, 1.0)",
        "isInPlay" : true,
        "isStrike" : false,
        "isBall" : false,
        "hasReview" : false
      },
      "count" : {
        "balls" : 0,
        "strikes" : 0
      },
      "pitchData" : {
        "strikeZoneTop" : 3.359,
        "strikeZoneBottom" : 1.7,
        "coordinates" : {
          "x" : 88.41,
          "y" : 114.84
        },
        "breaks" : { }
      },
      "hitData" : {
        "trajectory" : "line_drive",
        "hardness" : "medium",
        "location" : "7",
        "coordinates" : {
          "coordX" : 63.25,
          "coordY" : 117.47
        }
      },
      "index" : 0,
      "playId" : "02401986-0946-0003-000c-f08cd117d70a",
      "isPitch" : true,
      "type" : "pitch"
    } ],
    "atBatIndex" : 93
  },
  "scoringPlays" : [ 9, 14, 17, 42, 53, 58, 59, 70, 78, 79, 93 ],
  "playsByInning" : [ {
    "startIndex" : 0,
    "endIndex" : 5,
    "top" : [ 0, 1, 2 ],
    "bottom" : [ 3, 4, 5 ],
    "hits" : {
      "away" : [ {
        "team" : {
          "id" : 608,
          "name" : "DSL Cardinals",
          "link" : "/api/v1/teams/608",
          "allStarStatus" : "N"
        },
        "inning" : 1,
        "pitcher" : {
          "id" : 516710,
          "fullName" : "Siulman Lebron",
          "link" : "/api/v1/people/516710"
        },
        "batter" : {
          "id" : 542458,
          "fullName" : "Wader Perez",
          "link" : "/api/v1/people/542458"
        },
        "coordinates" : {
          "x" : 142.57,
          "y" : 175.7
        },
        "type" : "O",
        "description" : "Pop Out"
      }, {
        "team" : {
          "id" : 608,
          "name" : "DSL Cardinals",
          "link" : "/api/v1/teams/608",
          "allStarStatus" : "N"
        },
        "inning" : 1,
        "pitcher" : {
          "id" : 516710,
          "fullName" : "Siulman Lebron",
          "link" : "/api/v1/people/516710"
        },
        "batter" : {
          "id" : 542463,
          "fullName" : "Alex Castellano",
          "link" : "/api/v1/people/542463"
        },
        "coordinates" : {
          "x" : 132.53,
          "y" : 168.67
        },
        "type" : "O",
        "description" : "Groundout"
      }, {
        "team" : {
          "id" : 608,
          "name" : "DSL Cardinals",
          "link" : "/api/v1/teams/608",
          "allStarStatus" : "N"
        },
        "inning" : 1,
        "pitcher" : {
          "id" : 516710,
          "fullName" : "Siulman Lebron",
          "link" : "/api/v1/people/516710"
        },
        "batter" : {
          "id" : 516868,
          "fullName" : "Juan B Cabrera",
          "link" : "/api/v1/people/516868"
        },
        "coordinates" : {
          "x" : 135.54,
          "y" : 167.67
        },
        "type" : "O",
        "description" : "Groundout"
      } ],
      "home" : [ {
        "team" : {
          "id" : 623,
          "name" : "DSL Phillies",
          "link" : "/api/v1/teams/623",
          "allStarStatus" : "N"
        },
        "inning" : 1,
        "pitcher" : {
          "id" : 542203,
          "fullName" : "Angel De Jesus",
          "link" : "/api/v1/people/542203"
        },
        "batter" : {
          "id" : 516680,
          "fullName" : "Rudney Balentien",
          "link" : "/api/v1/people/516680"
        },
        "coordinates" : {
          "x" : 168.67,
          "y" : 109.44
        },
        "type" : "O",
        "description" : "Flyout"
      } ]
    }
  }, {
    "startIndex" : 6,
    "endIndex" : 18,
    "top" : [ 6, 7, 8, 9, 10 ],
    "bottom" : [ 11, 12, 13, 14, 15, 16, 17, 18 ],
    "hits" : {
      "away" : [ {
        "team" : {
          "id" : 608,
          "name" : "DSL Cardinals",
          "link" : "/api/v1/teams/608",
          "allStarStatus" : "N"
        },
        "inning" : 2,
        "pitcher" : {
          "id" : 516710,
          "fullName" : "Siulman Lebron",
          "link" : "/api/v1/people/516710"
        },
        "batter" : {
          "id" : 540938,
          "fullName" : "Luis Pimentel",
          "link" : "/api/v1/people/540938"
        },
        "coordinates" : {
          "x" : 69.28,
          "y" : 128.51
        },
        "type" : "H",
        "description" : "Single"
      }, {
        "team" : {
          "id" : 608,
          "name" : "DSL Cardinals",
          "link" : "/api/v1/teams/608",
          "allStarStatus" : "N"
        },
        "inning" : 2,
        "pitcher" : {
          "id" : 516710,
          "fullName" : "Siulman Lebron",
          "link" : "/api/v1/people/516710"
        },
        "batter" : {
          "id" : 542468,
          "fullName" : "Santo Sandoval",
          "link" : "/api/v1/people/542468"
        },
        "coordinates" : {
          "x" : 132.53,
          "y" : 166.67
        },
        "type" : "O",
        "description" : "Groundout"
      }, {
        "team" : {
          "id" : 608,
          "name" : "DSL Cardinals",
          "link" : "/api/v1/teams/608",
          "allStarStatus" : "N"
        },
        "inning" : 2,
        "pitcher" : {
          "id" : 516710,
          "fullName" : "Siulman Lebron",
          "link" : "/api/v1/people/516710"
        },
        "batter" : {
          "id" : 542248,
          "fullName" : "Bernardo Villar",
          "link" : "/api/v1/people/542248"
        },
        "coordinates" : {
          "x" : 145.58,
          "y" : 175.7
        },
        "type" : "E",
        "description" : "Field Error"
      }, {
        "team" : {
          "id" : 608,
          "name" : "DSL Cardinals",
          "link" : "/api/v1/teams/608",
          "allStarStatus" : "N"
        },
        "inning" : 2,
        "pitcher" : {
          "id" : 516710,
          "fullName" : "Siulman Lebron",
          "link" : "/api/v1/people/516710"
        },
        "batter" : {
          "id" : 542465,
          "fullName" : "Roberto Reyes",
          "link" : "/api/v1/people/542465"
        },
        "coordinates" : {
          "x" : 142.57,
          "y" : 174.7
        },
        "type" : "O",
        "description" : "Groundout"
      } ],
      "home" : [ {
        "team" : {
          "id" : 623,
          "name" : "DSL Phillies",
          "link" : "/api/v1/teams/623",
          "allStarStatus" : "N"
        },
        "inning" : 2,
        "pitcher" : {
          "id" : 542203,
          "fullName" : "Angel De Jesus",
          "link" : "/api/v1/people/542203"
        },
        "batter" : {
          "id" : 516760,
          "fullName" : "Luis Paulino",
          "link" : "/api/v1/people/516760"
        },
        "coordinates" : {
          "x" : 124.5,
          "y" : 58.23
        },
        "type" : "H",
        "description" : "Triple"
      }, {
        "team" : {
          "id" : 623,
          "name" : "DSL Phillies",
          "link" : "/api/v1/teams/623",
          "allStarStatus" : "N"
        },
        "inning" : 2,
        "pitcher" : {
          "id" : 542203,
          "fullName" : "Angel De Jesus",
          "link" : "/api/v1/people/542203"
        },
        "batter" : {
          "id" : 542758,
          "fullName" : "Carlos Best",
          "link" : "/api/v1/people/542758"
        },
        "coordinates" : {
          "x" : 87.35,
          "y" : 159.64
        },
        "type" : "O",
        "description" : "Pop Out"
      }, {
        "team" : {
          "id" : 623,
          "name" : "DSL Phillies",
          "link" : "/api/v1/teams/623",
          "allStarStatus" : "N"
        },
        "inning" : 2,
        "pitcher" : {
          "id" : 542203,
          "fullName" : "Angel De Jesus",
          "link" : "/api/v1/people/542203"
        },
        "batter" : {
          "id" : 542340,
          "fullName" : "Jonathan Villar",
          "link" : "/api/v1/people/542340"
        },
        "coordinates" : {
          "x" : 106.43,
          "y" : 168.67
        },
        "type" : "E",
        "description" : "Field Error"
      }, {
        "team" : {
          "id" : 623,
          "name" : "DSL Phillies",
          "link" : "/api/v1/teams/623",
          "allStarStatus" : "N"
        },
        "inning" : 2,
        "pitcher" : {
          "id" : 542203,
          "fullName" : "Angel De Jesus",
          "link" : "/api/v1/people/542203"
        },
        "batter" : {
          "id" : 542477,
          "fullName" : "Carlos Valenzuela",
          "link" : "/api/v1/people/542477"
        },
        "coordinates" : {
          "x" : 127.51,
          "y" : 141.57
        },
        "type" : "O",
        "description" : "Pop Out"
      } ]
    }
  }, {
    "startIndex" : 19,
    "endIndex" : 26,
    "top" : [ 19, 20, 21, 22 ],
    "bottom" : [ 23, 24, 25, 26 ],
    "hits" : {
      "away" : [ {
        "team" : {
          "id" : 608,
          "name" : "DSL Cardinals",
          "link" : "/api/v1/teams/608",
          "allStarStatus" : "N"
        },
        "inning" : 3,
        "pitcher" : {
          "id" : 516710,
          "fullName" : "Siulman Lebron",
          "link" : "/api/v1/people/516710"
        },
        "batter" : {
          "id" : 516658,
          "fullName" : "Marcos Martinez",
          "link" : "/api/v1/people/516658"
        },
        "coordinates" : {
          "x" : 100.4,
          "y" : 177.71
        },
        "type" : "O",
        "description" : "Groundout"
      }, {
        "team" : {
          "id" : 608,
          "name" : "DSL Cardinals",
          "link" : "/api/v1/teams/608",
          "allStarStatus" : "N"
        },
        "inning" : 3,
        "pitcher" : {
          "id" : 516710,
          "fullName" : "Siulman Lebron",
          "link" : "/api/v1/people/516710"
        },
        "batter" : {
          "id" : 542458,
          "fullName" : "Wader Perez",
          "link" : "/api/v1/people/542458"
        },
        "coordinates" : {
          "x" : 151.61,
          "y" : 115.46
        },
        "type" : "H",
        "description" : "Single"
      }, {
        "team" : {
          "id" : 608,
          "name" : "DSL Cardinals",
          "link" : "/api/v1/teams/608",
          "allStarStatus" : "N"
        },
        "inning" : 3,
        "pitcher" : {
          "id" : 516710,
          "fullName" : "Siulman Lebron",
          "link" : "/api/v1/people/516710"
        },
        "batter" : {
          "id" : 542463,
          "fullName" : "Alex Castellano",
          "link" : "/api/v1/people/542463"
        },
        "coordinates" : {
          "x" : 94.38,
          "y" : 192.77
        },
        "type" : "O",
        "description" : "Pop Out"
      }, {
        "team" : {
          "id" : 608,
          "name" : "DSL Cardinals",
          "link" : "/api/v1/teams/608",
          "allStarStatus" : "N"
        },
        "inning" : 3,
        "pitcher" : {
          "id" : 516710,
          "fullName" : "Siulman Lebron",
          "link" : "/api/v1/people/516710"
        },
        "batter" : {
          "id" : 516868,
          "fullName" : "Juan B Cabrera",
          "link" : "/api/v1/people/516868"
        },
        "coordinates" : {
          "x" : 138.55,
          "y" : 166.67
        },
        "type" : "O",
        "description" : "Forceout"
      } ],
      "home" : [ {
        "team" : {
          "id" : 623,
          "name" : "DSL Phillies",
          "link" : "/api/v1/teams/623",
          "allStarStatus" : "N"
        },
        "inning" : 3,
        "pitcher" : {
          "id" : 542203,
          "fullName" : "Angel De Jesus",
          "link" : "/api/v1/people/542203"
        },
        "batter" : {
          "id" : 516729,
          "fullName" : "Pedro Aguilar",
          "link" : "/api/v1/people/516729"
        },
        "coordinates" : {
          "x" : 67.27,
          "y" : 111.45
        },
        "type" : "O",
        "description" : "Flyout"
      }, {
        "team" : {
          "id" : 623,
          "name" : "DSL Phillies",
          "link" : "/api/v1/teams/623",
          "allStarStatus" : "N"
        },
        "inning" : 3,
        "pitcher" : {
          "id" : 542203,
          "fullName" : "Angel De Jesus",
          "link" : "/api/v1/people/542203"
        },
        "batter" : {
          "id" : 542479,
          "fullName" : "Geancarlo Mendez",
          "link" : "/api/v1/people/542479"
        },
        "coordinates" : {
          "x" : 83.33,
          "y" : 131.53
        },
        "type" : "H",
        "description" : "Single"
      } ]
    }
  }, {
    "startIndex" : 27,
    "endIndex" : 33,
    "top" : [ 27, 28, 29 ],
    "bottom" : [ 30, 31, 32, 33 ],
    "hits" : {
      "away" : [ {
        "team" : {
          "id" : 608,
          "name" : "DSL Cardinals",
          "link" : "/api/v1/teams/608",
          "allStarStatus" : "N"
        },
        "inning" : 4,
        "pitcher" : {
          "id" : 516710,
          "fullName" : "Siulman Lebron",
          "link" : "/api/v1/people/516710"
        },
        "batter" : {
          "id" : 542233,
          "fullName" : "Audry Perez",
          "link" : "/api/v1/people/542233"
        },
        "coordinates" : {
          "x" : 103.41,
          "y" : 176.71
        },
        "type" : "O",
        "description" : "Groundout"
      }, {
        "team" : {
          "id" : 608,
          "name" : "DSL Cardinals",
          "link" : "/api/v1/teams/608",
          "allStarStatus" : "N"
        },
        "inning" : 4,
        "pitcher" : {
          "id" : 516710,
          "fullName" : "Siulman Lebron",
          "link" : "/api/v1/people/516710"
        },
        "batter" : {
          "id" : 542468,
          "fullName" : "Santo Sandoval",
          "link" : "/api/v1/people/542468"
        },
        "coordinates" : {
          "x" : 153.61,
          "y" : 191.77
        },
        "type" : "O",
        "description" : "Pop Out"
      } ],
      "home" : [ {
        "team" : {
          "id" : 623,
          "name" : "DSL Phillies",
          "link" : "/api/v1/teams/623",
          "allStarStatus" : "N"
        },
        "inning" : 4,
        "pitcher" : {
          "id" : 542203,
          "fullName" : "Angel De Jesus",
          "link" : "/api/v1/people/542203"
        },
        "batter" : {
          "id" : 516760,
          "fullName" : "Luis Paulino",
          "link" : "/api/v1/people/516760"
        },
        "coordinates" : {
          "x" : 99.4,
          "y" : 146.59
        },
        "type" : "O",
        "description" : "Pop Out"
      }, {
        "team" : {
          "id" : 623,
          "name" : "DSL Phillies",
          "link" : "/api/v1/teams/623",
          "allStarStatus" : "N"
        },
        "inning" : 4,
        "pitcher" : {
          "id" : 542203,
          "fullName" : "Angel De Jesus",
          "link" : "/api/v1/people/542203"
        },
        "batter" : {
          "id" : 542339,
          "fullName" : "Jose Trinidad",
          "link" : "/api/v1/people/542339"
        },
        "coordinates" : {
          "x" : 106.43,
          "y" : 169.68
        },
        "type" : "O",
        "description" : "Groundout"
      }, {
        "team" : {
          "id" : 623,
          "name" : "DSL Phillies",
          "link" : "/api/v1/teams/623",
          "allStarStatus" : "N"
        },
        "inning" : 4,
        "pitcher" : {
          "id" : 542203,
          "fullName" : "Angel De Jesus",
          "link" : "/api/v1/people/542203"
        },
        "batter" : {
          "id" : 542340,
          "fullName" : "Jonathan Villar",
          "link" : "/api/v1/people/542340"
        },
        "coordinates" : {
          "x" : 134.54,
          "y" : 163.65
        },
        "type" : "O",
        "description" : "Groundout"
      } ]
    }
  }, {
    "startIndex" : 34,
    "endIndex" : 39,
    "top" : [ 34, 35, 36 ],
    "bottom" : [ 37, 38, 39 ],
    "hits" : {
      "away" : [ {
        "team" : {
          "id" : 608,
          "name" : "DSL Cardinals",
          "link" : "/api/v1/teams/608",
          "allStarStatus" : "N"
        },
        "inning" : 5,
        "pitcher" : {
          "id" : 516710,
          "fullName" : "Siulman Lebron",
          "link" : "/api/v1/people/516710"
        },
        "batter" : {
          "id" : 542248,
          "fullName" : "Bernardo Villar",
          "link" : "/api/v1/people/542248"
        },
        "coordinates" : {
          "x" : 135.54,
          "y" : 136.55
        },
        "type" : "O",
        "description" : "Pop Out"
      }, {
        "team" : {
          "id" : 608,
          "name" : "DSL Cardinals",
          "link" : "/api/v1/teams/608",
          "allStarStatus" : "N"
        },
        "inning" : 5,
        "pitcher" : {
          "id" : 516710,
          "fullName" : "Siulman Lebron",
          "link" : "/api/v1/people/516710"
        },
        "batter" : {
          "id" : 542465,
          "fullName" : "Roberto Reyes",
          "link" : "/api/v1/people/542465"
        },
        "coordinates" : {
          "x" : 108.43,
          "y" : 167.67
        },
        "type" : "O",
        "description" : "Groundout"
      } ],
      "home" : [ {
        "team" : {
          "id" : 623,
          "name" : "DSL Phillies",
          "link" : "/api/v1/teams/623",
          "allStarStatus" : "N"
        },
        "inning" : 5,
        "pitcher" : {
          "id" : 542241,
          "fullName" : "Eddy Rivera",
          "link" : "/api/v1/people/542241"
        },
        "batter" : {
          "id" : 516680,
          "fullName" : "Rudney Balentien",
          "link" : "/api/v1/people/516680"
        },
        "coordinates" : {
          "x" : 97.39,
          "y" : 174.7
        },
        "type" : "E",
        "description" : "Field Error"
      } ]
    }
  }, {
    "startIndex" : 40,
    "endIndex" : 47,
    "top" : [ 40, 41, 42, 43 ],
    "bottom" : [ 44, 45, 46, 47 ],
    "hits" : {
      "away" : [ {
        "team" : {
          "id" : 608,
          "name" : "DSL Cardinals",
          "link" : "/api/v1/teams/608",
          "allStarStatus" : "N"
        },
        "inning" : 6,
        "pitcher" : {
          "id" : 516710,
          "fullName" : "Siulman Lebron",
          "link" : "/api/v1/people/516710"
        },
        "batter" : {
          "id" : 542458,
          "fullName" : "Wader Perez",
          "link" : "/api/v1/people/542458"
        },
        "coordinates" : {
          "x" : 108.43,
          "y" : 165.66
        },
        "type" : "O",
        "description" : "Groundout"
      }, {
        "team" : {
          "id" : 608,
          "name" : "DSL Cardinals",
          "link" : "/api/v1/teams/608",
          "allStarStatus" : "N"
        },
        "inning" : 6,
        "pitcher" : {
          "id" : 516710,
          "fullName" : "Siulman Lebron",
          "link" : "/api/v1/people/516710"
        },
        "batter" : {
          "id" : 542463,
          "fullName" : "Alex Castellano",
          "link" : "/api/v1/people/542463"
        },
        "coordinates" : {
          "x" : 125.5,
          "y" : 57.23
        },
        "type" : "H",
        "description" : "Triple"
      }, {
        "team" : {
          "id" : 608,
          "name" : "DSL Cardinals",
          "link" : "/api/v1/teams/608",
          "allStarStatus" : "N"
        },
        "inning" : 6,
        "pitcher" : {
          "id" : 516710,
          "fullName" : "Siulman Lebron",
          "link" : "/api/v1/people/516710"
        },
        "batter" : {
          "id" : 516868,
          "fullName" : "Juan B Cabrera",
          "link" : "/api/v1/people/516868"
        },
        "coordinates" : {
          "x" : 106.43,
          "y" : 164.66
        },
        "type" : "O",
        "description" : "Groundout"
      }, {
        "team" : {
          "id" : 608,
          "name" : "DSL Cardinals",
          "link" : "/api/v1/teams/608",
          "allStarStatus" : "N"
        },
        "inning" : 6,
        "pitcher" : {
          "id" : 516710,
          "fullName" : "Siulman Lebron",
          "link" : "/api/v1/people/516710"
        },
        "batter" : {
          "id" : 542233,
          "fullName" : "Audry Perez",
          "link" : "/api/v1/people/542233"
        },
        "coordinates" : {
          "x" : 112.45,
          "y" : 111.45
        },
        "type" : "O",
        "description" : "Flyout"
      } ],
      "home" : [ {
        "team" : {
          "id" : 623,
          "name" : "DSL Phillies",
          "link" : "/api/v1/teams/623",
          "allStarStatus" : "N"
        },
        "inning" : 6,
        "pitcher" : {
          "id" : 542241,
          "fullName" : "Eddy Rivera",
          "link" : "/api/v1/people/542241"
        },
        "batter" : {
          "id" : 542479,
          "fullName" : "Geancarlo Mendez",
          "link" : "/api/v1/people/542479"
        },
        "coordinates" : {
          "x" : 57.23,
          "y" : 116.47
        },
        "type" : "O",
        "description" : "Flyout"
      }, {
        "team" : {
          "id" : 623,
          "name" : "DSL Phillies",
          "link" : "/api/v1/teams/623",
          "allStarStatus" : "N"
        },
        "inning" : 6,
        "pitcher" : {
          "id" : 542241,
          "fullName" : "Eddy Rivera",
          "link" : "/api/v1/people/542241"
        },
        "batter" : {
          "id" : 516760,
          "fullName" : "Luis Paulino",
          "link" : "/api/v1/people/516760"
        },
        "coordinates" : {
          "x" : 111.45,
          "y" : 165.66
        },
        "type" : "O",
        "description" : "Groundout"
      } ]
    }
  }, {
    "startIndex" : 48,
    "endIndex" : 62,
    "top" : [ 48, 49, 50, 51, 52, 53, 54 ],
    "bottom" : [ 55, 56, 57, 58, 59, 60, 61, 62 ],
    "hits" : {
      "away" : [ {
        "team" : {
          "id" : 608,
          "name" : "DSL Cardinals",
          "link" : "/api/v1/teams/608",
          "allStarStatus" : "N"
        },
        "inning" : 7,
        "pitcher" : {
          "id" : 501651,
          "fullName" : "Raudy Sandoval",
          "link" : "/api/v1/people/501651"
        },
        "batter" : {
          "id" : 540938,
          "fullName" : "Luis Pimentel",
          "link" : "/api/v1/people/540938"
        },
        "coordinates" : {
          "x" : 125.5,
          "y" : 97.39
        },
        "type" : "O",
        "description" : "Flyout"
      }, {
        "team" : {
          "id" : 608,
          "name" : "DSL Cardinals",
          "link" : "/api/v1/teams/608",
          "allStarStatus" : "N"
        },
        "inning" : 7,
        "pitcher" : {
          "id" : 501651,
          "fullName" : "Raudy Sandoval",
          "link" : "/api/v1/people/501651"
        },
        "batter" : {
          "id" : 542248,
          "fullName" : "Bernardo Villar",
          "link" : "/api/v1/people/542248"
        },
        "coordinates" : {
          "x" : 164.66,
          "y" : 109.44
        },
        "type" : "O",
        "description" : "Flyout"
      }, {
        "team" : {
          "id" : 608,
          "name" : "DSL Cardinals",
          "link" : "/api/v1/teams/608",
          "allStarStatus" : "N"
        },
        "inning" : 7,
        "pitcher" : {
          "id" : 525734,
          "fullName" : "Gabriel Arias",
          "link" : "/api/v1/people/525734"
        },
        "batter" : {
          "id" : 542458,
          "fullName" : "Wader Perez",
          "link" : "/api/v1/people/542458"
        },
        "coordinates" : {
          "x" : 172.69,
          "y" : 111.45
        },
        "type" : "H",
        "description" : "Single"
      } ],
      "home" : [ {
        "team" : {
          "id" : 623,
          "name" : "DSL Phillies",
          "link" : "/api/v1/teams/623",
          "allStarStatus" : "N"
        },
        "inning" : 7,
        "pitcher" : {
          "id" : 542246,
          "fullName" : "Randy Santos",
          "link" : "/api/v1/people/542246"
        },
        "batter" : {
          "id" : 542340,
          "fullName" : "Jonathan Villar",
          "link" : "/api/v1/people/542340"
        },
        "coordinates" : {
          "x" : 69.28,
          "y" : 122.49
        },
        "type" : "H",
        "description" : "Single"
      }, {
        "team" : {
          "id" : 623,
          "name" : "DSL Phillies",
          "link" : "/api/v1/teams/623",
          "allStarStatus" : "N"
        },
        "inning" : 7,
        "pitcher" : {
          "id" : 542246,
          "fullName" : "Randy Santos",
          "link" : "/api/v1/people/542246"
        },
        "batter" : {
          "id" : 542477,
          "fullName" : "Carlos Valenzuela",
          "link" : "/api/v1/people/542477"
        },
        "coordinates" : {
          "x" : 144.58,
          "y" : 176.71
        },
        "type" : "O",
        "description" : "Forceout"
      }, {
        "team" : {
          "id" : 623,
          "name" : "DSL Phillies",
          "link" : "/api/v1/teams/623",
          "allStarStatus" : "N"
        },
        "inning" : 7,
        "pitcher" : {
          "id" : 542246,
          "fullName" : "Randy Santos",
          "link" : "/api/v1/people/542246"
        },
        "batter" : {
          "id" : 516758,
          "fullName" : "Miguel Alvarez",
          "link" : "/api/v1/people/516758"
        },
        "coordinates" : {
          "x" : 124.5,
          "y" : 104.42
        },
        "type" : "H",
        "description" : "Single"
      }, {
        "team" : {
          "id" : 623,
          "name" : "DSL Phillies",
          "link" : "/api/v1/teams/623",
          "allStarStatus" : "N"
        },
        "inning" : 7,
        "pitcher" : {
          "id" : 542246,
          "fullName" : "Randy Santos",
          "link" : "/api/v1/people/542246"
        },
        "batter" : {
          "id" : 516729,
          "fullName" : "Pedro Aguilar",
          "link" : "/api/v1/people/516729"
        },
        "coordinates" : {
          "x" : 96.39,
          "y" : 146.59
        },
        "type" : "O",
        "description" : "Pop Out"
      } ]
    }
  }, {
    "startIndex" : 63,
    "endIndex" : 72,
    "top" : [ 63, 64, 65, 66 ],
    "bottom" : [ 67, 68, 69, 70, 71, 72 ],
    "hits" : {
      "away" : [ {
        "team" : {
          "id" : 608,
          "name" : "DSL Cardinals",
          "link" : "/api/v1/teams/608",
          "allStarStatus" : "N"
        },
        "inning" : 8,
        "pitcher" : {
          "id" : 525734,
          "fullName" : "Gabriel Arias",
          "link" : "/api/v1/people/525734"
        },
        "batter" : {
          "id" : 542233,
          "fullName" : "Audry Perez",
          "link" : "/api/v1/people/542233"
        },
        "coordinates" : {
          "x" : 106.43,
          "y" : 165.66
        },
        "type" : "O",
        "description" : "Groundout"
      }, {
        "team" : {
          "id" : 608,
          "name" : "DSL Cardinals",
          "link" : "/api/v1/teams/608",
          "allStarStatus" : "N"
        },
        "inning" : 8,
        "pitcher" : {
          "id" : 525734,
          "fullName" : "Gabriel Arias",
          "link" : "/api/v1/people/525734"
        },
        "batter" : {
          "id" : 540938,
          "fullName" : "Luis Pimentel",
          "link" : "/api/v1/people/540938"
        },
        "coordinates" : {
          "x" : 120.48,
          "y" : 107.43
        },
        "type" : "O",
        "description" : "Flyout"
      } ],
      "home" : [ {
        "team" : {
          "id" : 623,
          "name" : "DSL Phillies",
          "link" : "/api/v1/teams/623",
          "allStarStatus" : "N"
        },
        "inning" : 8,
        "pitcher" : {
          "id" : 542523,
          "fullName" : "Amaury Castillo",
          "link" : "/api/v1/people/542523"
        },
        "batter" : {
          "id" : 542477,
          "fullName" : "Carlos Valenzuela",
          "link" : "/api/v1/people/542477"
        },
        "coordinates" : {
          "x" : 72.29,
          "y" : 118.47
        },
        "type" : "H",
        "description" : "Single"
      }, {
        "team" : {
          "id" : 623,
          "name" : "DSL Phillies",
          "link" : "/api/v1/teams/623",
          "allStarStatus" : "N"
        },
        "inning" : 8,
        "pitcher" : {
          "id" : 542523,
          "fullName" : "Amaury Castillo",
          "link" : "/api/v1/people/542523"
        },
        "batter" : {
          "id" : 516729,
          "fullName" : "Pedro Aguilar",
          "link" : "/api/v1/people/516729"
        },
        "coordinates" : {
          "x" : 103.41,
          "y" : 176.71
        },
        "type" : "O",
        "description" : "Pop Out"
      } ]
    }
  }, {
    "startIndex" : 73,
    "endIndex" : 86,
    "top" : [ 73, 74, 75, 76, 77, 78, 79, 80 ],
    "bottom" : [ 81, 82, 83, 84, 85, 86 ],
    "hits" : {
      "away" : [ {
        "team" : {
          "id" : 608,
          "name" : "DSL Cardinals",
          "link" : "/api/v1/teams/608",
          "allStarStatus" : "N"
        },
        "inning" : 9,
        "pitcher" : {
          "id" : 469192,
          "fullName" : "Joel Martinez",
          "link" : "/api/v1/people/469192"
        },
        "batter" : {
          "id" : 516868,
          "fullName" : "Juan B Cabrera",
          "link" : "/api/v1/people/516868"
        },
        "coordinates" : {
          "x" : 69.28,
          "y" : 119.48
        },
        "type" : "H",
        "description" : "Single"
      }, {
        "team" : {
          "id" : 608,
          "name" : "DSL Cardinals",
          "link" : "/api/v1/teams/608",
          "allStarStatus" : "N"
        },
        "inning" : 9,
        "pitcher" : {
          "id" : 469192,
          "fullName" : "Joel Martinez",
          "link" : "/api/v1/people/469192"
        },
        "batter" : {
          "id" : 542233,
          "fullName" : "Audry Perez",
          "link" : "/api/v1/people/542233"
        },
        "coordinates" : {
          "x" : 56.22,
          "y" : 121.49
        },
        "type" : "H",
        "description" : "Single"
      }, {
        "team" : {
          "id" : 608,
          "name" : "DSL Cardinals",
          "link" : "/api/v1/teams/608",
          "allStarStatus" : "N"
        },
        "inning" : 9,
        "pitcher" : {
          "id" : 469192,
          "fullName" : "Joel Martinez",
          "link" : "/api/v1/people/469192"
        },
        "batter" : {
          "id" : 540938,
          "fullName" : "Luis Pimentel",
          "link" : "/api/v1/people/540938"
        },
        "coordinates" : {
          "x" : 127.51,
          "y" : 168.67
        },
        "type" : "O",
        "description" : "Groundout"
      } ],
      "home" : [ {
        "team" : {
          "id" : 623,
          "name" : "DSL Phillies",
          "link" : "/api/v1/teams/623",
          "allStarStatus" : "N"
        },
        "inning" : 9,
        "pitcher" : {
          "id" : 542524,
          "fullName" : "Jose Almarante",
          "link" : "/api/v1/people/542524"
        },
        "batter" : {
          "id" : 501517,
          "fullName" : "Emmanuel Checo",
          "link" : "/api/v1/people/501517"
        },
        "coordinates" : {
          "x" : 120.48,
          "y" : 198.8
        },
        "type" : "O",
        "description" : "Sac Bunt"
      } ]
    }
  }, {
    "startIndex" : 87,
    "endIndex" : 93,
    "top" : [ 87, 88, 89 ],
    "bottom" : [ 90, 91, 92, 93 ],
    "hits" : {
      "away" : [ {
        "team" : {
          "id" : 608,
          "name" : "DSL Cardinals",
          "link" : "/api/v1/teams/608",
          "allStarStatus" : "N"
        },
        "inning" : 10,
        "pitcher" : {
          "id" : 545002,
          "fullName" : "Juan Sepulveda",
          "link" : "/api/v1/people/545002"
        },
        "batter" : {
          "id" : 542468,
          "fullName" : "Santo Sandoval",
          "link" : "/api/v1/people/542468"
        },
        "coordinates" : {
          "x" : 53.21,
          "y" : 115.46
        },
        "type" : "O",
        "description" : "Flyout"
      } ],
      "home" : [ {
        "team" : {
          "id" : 623,
          "name" : "DSL Phillies",
          "link" : "/api/v1/teams/623",
          "allStarStatus" : "N"
        },
        "inning" : 10,
        "pitcher" : {
          "id" : 542524,
          "fullName" : "Jose Almarante",
          "link" : "/api/v1/people/542524"
        },
        "batter" : {
          "id" : 542477,
          "fullName" : "Carlos Valenzuela",
          "link" : "/api/v1/people/542477"
        },
        "coordinates" : {
          "x" : 127.51,
          "y" : 189.76
        },
        "type" : "E",
        "description" : "Field Error"
      }, {
        "team" : {
          "id" : 623,
          "name" : "DSL Phillies",
          "link" : "/api/v1/teams/623",
          "allStarStatus" : "N"
        },
        "inning" : 10,
        "pitcher" : {
          "id" : 542524,
          "fullName" : "Jose Almarante",
          "link" : "/api/v1/people/542524"
        },
        "batter" : {
          "id" : 516758,
          "fullName" : "Miguel Alvarez",
          "link" : "/api/v1/people/516758"
        },
        "coordinates" : {
          "x" : 76.31,
          "y" : 116.47
        },
        "type" : "H",
        "description" : "Single"
      }, {
        "team" : {
          "id" : 623,
          "name" : "DSL Phillies",
          "link" : "/api/v1/teams/623",
          "allStarStatus" : "N"
        },
        "inning" : 10,
        "pitcher" : {
          "id" : 542524,
          "fullName" : "Jose Almarante",
          "link" : "/api/v1/people/542524"
        },
        "batter" : {
          "id" : 542479,
          "fullName" : "Geancarlo Mendez",
          "link" : "/api/v1/people/542479"
        },
        "coordinates" : {
          "x" : 63.25,
          "y" : 117.47
        },
        "type" : "H",
        "description" : "Single"
      } ]
    }
  } ]
}"#;