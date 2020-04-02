/// API Bindings for the MLB Boxscore API hosted at http://statsapi.mlb.com/api/v1/game/{game_pk}/boxscore.<br/>
/// All data are subject to MLB Advanced Media copyright
/// 
/// These data are important for setting the intitial defense, as well as specific metadata for the game such as weather, umpires and attendance.
/// 
/// 
/// 


use serde::{Deserialize, Serialize};
use std::collections::{HashMap};
use regex::Regex;

//Horrible hack to allow us to pull in the players. Somewhat brittle, but can't figure
//out a better way to do it
pub (crate) fn fix_boxscore (boxscore: &str) -> String {
  let regex = Regex::new(r#""ID\d{6}" : "#).unwrap();
  let regex_ws = Regex::new(r"\s+").unwrap();

  regex
    .replace_all(&regex_ws.replace_all(boxscore, " "), "")
    .replace("players\" : {", "players\" : [")
    .replace("}, \"batters\"", "], \"batters\"")
}

/// BoxScoreData cannot be built from deserialized data since they don't contain the game_pk.
/// BoxScore will automatically be converted from the BoxScoreDe, along with all the transformation
/// logic neccessary. 
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BoxScoreData {
  pub game_pk: u32,
  pub boxscore_data: BoxScore,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BoxScore {
  pub attendance: Option<u32>,
  pub first_pitch: Option<f32>,
  pub game_weather_temp_f: Option<u8>,
  pub game_weather_temp_c: Option<i8>,
  pub game_weather_condition: Option<WeatherCondition>,
  pub game_wind_speed_mph: Option<u8>,
  pub game_wind_direction: Option<WindDirection>,
  
  pub home_team_id: u32,
  pub away_team_id: u32,
  pub home_league_id: Option<u32>,
  pub home_league_name: Option<String>,
  pub away_league_id: Option<u32>,
  pub away_league_name: Option<String>,

  pub home_division_id: Option<u32>,
  pub away_division_id: Option<u32>,
  pub home_division_name: Option<String>,
  pub away_division_name: Option<String>,

  pub home_sport_id: u32,
  pub away_sport_id: u32,
  pub home_parent_team_id: u32,
  pub away_parent_team_id: u32,
  
  pub hp_umpire_id: Option<u32>,

  pub home_players: Vec<Player>,
  pub away_players: Vec<Player>,

  pub home_defense: Defense,
  pub away_defense: Defense,

}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Defense {
  pub catcher: Option<u32>,
  pub first_base: Option<u32>,
  pub second_base: Option<u32>,
  pub short_stop: Option<u32>,
  pub third_base: Option<u32>,
  pub left_field: Option<u32>,
  pub right_field: Option<u32>,
  pub center_field: Option<u32>,
  pub pitcher: Option<u32>,
  pub designated_hitter: Option<u32>,
}

impl From <Vec<Player>> for Defense {
  fn from (players: Vec<Player>) -> Defense {
    
    /// Each player that takes the field has their positions listed in order in the "all_positions" field.
    /// In order to find the starting defense, we first look through all the people in the starting batting order
    /// ending in "00" then fill in the gaps with those ending in "01". This should cover all the starting defenses.
    /// In case there were multiple pinch hitters before anyone took the field, we could trivially expand this logic to
    /// include batters that end in "02".
    fn get_player (pos: Pos, players_00: &HashMap<Pos, u32>, players_01: &HashMap<Pos, u32>) -> Option<u32> {

      let player = players_00.get(&pos).copied();

      match player {
        None => players_01.get(&pos).copied(),
        _ => player,
      }

    }


    let players_00: HashMap<Pos, u32> = players
      .clone()
      .into_iter()
      .filter(|player| player.batting_order.is_some())
      .filter(|player| player.batting_order.unwrap() % 100 == 0)
      .map(|player| (player.position, player.id))
      .collect();
    
      let players_01: HashMap<Pos, u32> = players
      .into_iter()
      .filter(|player| player.batting_order.is_some())
      .filter(|player| player.batting_order.unwrap() % 100 == 1)
      .map(|player| (player.position, player.id))
      .collect();
    
    Defense {
      catcher:             get_player(Pos::Catcher,         &players_00, &players_01),
      first_base:          get_player(Pos::FirstBase,       &players_00, &players_01),
      second_base:         get_player(Pos::SecondBase,      &players_00, &players_01),
      short_stop:          get_player(Pos::ShortStop,       &players_00, &players_01),
      third_base:          get_player(Pos::ThirdBase,       &players_00, &players_01),
      left_field:          get_player(Pos::LeftField,       &players_00, &players_01),
      right_field:         get_player(Pos::RightField,      &players_00, &players_01),
      center_field:        get_player(Pos::CenterField,     &players_00, &players_01),
      pitcher:             get_player(Pos::StartingPitcher, &players_00, &players_01),
      designated_hitter:   get_player(Pos::DesignatedHitter, &players_00, &players_01),
    }
  }
}

fn f_to_c (t: u8) -> i8 {
  ((t - 32) as f32 * (5f32 / 9f32)) as i8
}

impl From <BoxScoreDe> for BoxScore {
  fn from (box_score: BoxScoreDe) -> BoxScore {
    
    let info: HashMap<String, Option<String>> = box_score.info
        .into_iter()
        .map(|i| (i.label, i.value))
        .collect();

    let weather = info.get("Weather").unwrap_or(&None);
    let (game_weather_temp_f, game_weather_temp_c, game_weather_condition) = match weather {
      Some (weather) =>  
        {
          let temp = weather.split(" ").nth(0).unwrap_or("");
          let (deg_f, deg_c) = match temp.parse::<u8>() {
            Ok(temp) => (Some(temp), Some (f_to_c(temp))),
            _ => (None, None),
          };
          let cond = match weather.split(",").nth(1) {
            Some(cond) => Some (cond.into()),
            _ => None,
          };

          (deg_f, deg_c, cond)
        },
      _ => (None, None, None),
    };

    let wind = info.get("Wind").unwrap_or(&None);

    let (game_wind_speed_mph, game_wind_direction) = match wind {
      Some (wind) =>
        {
          let wind_speed = wind.split(" ").nth(0).unwrap_or("").parse::<u8>().ok();

          let wind_dir = match wind.split(",").nth(1) {
            Some (dir) => Some (dir.into()),
            _ => None,
          };
          (wind_speed, wind_dir)
        },
        _ => (None, None),
    };

    let attendance = info
              .get("Att")
              .unwrap_or(&None)
              .to_owned()
              .unwrap_or_default()
              .replace(",", "")
              .replace(".", "")
              .parse::<u32>()
              .ok();

    let first_pitch_text = info.get("First pitch").unwrap_or(&None).to_owned().unwrap_or_default();
    let first_pitch_vec: Vec<&str> = first_pitch_text.split(" ").collect();
    let hours_minutes: Vec<&str> = first_pitch_vec[0].split(":").collect();

    
    let first_pitch = match hours_minutes.len() {
      2 => {
        let am_pm = first_pitch_vec[1].replace(".","").to_lowercase();
        let first_pitch_f32 = hours_minutes[0].parse::<f32>().unwrap_or(0f32) 
                        + hours_minutes[1].parse::<f32>().unwrap_or(0f32)/60f32 
                        + match am_pm.as_str() {
                          "pm" => 12f32,
                          "am" | _ => 0f32,
                        };
        match first_pitch_f32 as u8 {
          0 => None,
          _ => Some(first_pitch_f32),
        }
      },
      _ => None
    };

    let hp_umpire_id = box_score.officials.iter()
                        .filter(|ump| ump.official_type == OfficialType::HomePlate)
                        .map(|ump| ump.official.id)
                        .nth(0)
                        ;

    let home_players = player_id_to_player(box_score.teams.home.players);
    let away_players = player_id_to_player(box_score.teams.away.players);

    let home_defense: Defense = home_players.clone().into();
    let away_defense: Defense = away_players.clone().into();

    let (home_division_id, home_division_name) = match box_score.teams.home.team.division {
      Some (div) => (Some(div.id), Some(div.name)),
      None => (None, None),
    };

    let (away_division_id, away_division_name) = match box_score.teams.away.team.division {
      Some (div) => (Some(div.id), Some(div.name)),
      None => (None, None),
    };

    BoxScore {
      game_weather_temp_f,
      game_weather_temp_c,
      game_weather_condition,
      game_wind_speed_mph,
      game_wind_direction,
      attendance,
      first_pitch,
      home_team_id: box_score.teams.home.team.id,
      away_team_id: box_score.teams.away.team.id,
      home_league_id: box_score.teams.home.team.league.id,
      home_league_name: box_score.teams.home.team.league.name,
      away_league_id: box_score.teams.away.team.league.id,
      away_league_name: box_score.teams.away.team.league.name,
      home_division_id,
      home_division_name,
      away_division_id,
      away_division_name,
      
      home_sport_id: box_score.teams.home.team.sport.id,
      away_sport_id: box_score.teams.away.team.sport.id,
      home_parent_team_id: box_score.teams.home.team.parent_org_id.unwrap_or(box_score.teams.home.team.id),
      away_parent_team_id: box_score.teams.away.team.parent_org_id.unwrap_or(box_score.teams.away.team.id),
      hp_umpire_id,
      home_defense,
      away_defense,
      home_players,
      away_players,
    }
  }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum WeatherCondition {
  Clear,
  Cloudy,
  Dome,
  Drizzle,
  Overcast,
  PartlyCloudy,
  Rain,
  RoofClosed,
  Snow,
  Sunny,
  Other,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum WindDirection {
  Calm,
  InFromCF,
  InFromLF,
  InFromRF,
  Indoors,
  LTOR,
  None,
  OutToCF,
  OutToLF,
  OutToRF,
  RTOL,
  Varies,
  Other
}

impl From<&str> for WeatherCondition {
  fn from (cond: &str) -> WeatherCondition {
    match cond.to_lowercase().trim().trim_end_matches(".") {
      "clear" => WeatherCondition::Clear,
      "cloudy" => WeatherCondition::Cloudy,
      "dome" => WeatherCondition::Dome,
      "drizzle" => WeatherCondition::Drizzle,
      "overcast" => WeatherCondition::Overcast,
      "partly cloudy" => WeatherCondition::PartlyCloudy,
      "rain" => WeatherCondition::Rain,
      "roof closed" => WeatherCondition::RoofClosed,
      "snow" => WeatherCondition::Snow,
      "sunny" => WeatherCondition::Sunny,
      _ => WeatherCondition::Other,
    }
  }
}

impl From<&str> for WindDirection {
  fn from (cond: &str) -> WindDirection {
    match cond.to_lowercase().trim().trim_end_matches(".") {
      "calm" => WindDirection::Calm,
      "in from cf" => WindDirection::InFromCF,
      "in from lf" => WindDirection::InFromLF,
      "in from rf" => WindDirection::InFromRF,
      "indoors" => WindDirection::Indoors,
      "l to r" => WindDirection::LTOR,
      "none" => WindDirection::None,
      "out to cf" => WindDirection::OutToCF,
      "out to lf" => WindDirection::OutToLF,
      "out to rf" => WindDirection::OutToRF,
      "r to l" => WindDirection::RTOL,
      "varies" => WindDirection::Varies,
      _ => WindDirection::Other,
    }
  }
}


#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub (crate) struct BoxScoreDe {
  pub (crate) teams: Teams,
  pub (crate) info: Vec<Info>,
  pub (crate) officials: Vec<Officials>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub (crate) struct Officials {
  pub (crate) official: Official,
  pub (crate) official_type: OfficialType,
}

#[derive(Deserialize, Debug)]
pub (crate) struct Official {
  pub (crate) id: u32,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub (crate) enum OfficialType {
  #[serde(alias="Home Plate")]
  HomePlate,
  //we only really care about the HP Umpire, however, we want the program to panic if we see
  //an umpire type that we don't expect
  #[serde(alias="First Base")]
  FirstBase,
  #[serde(alias="Second Base")]
  SecondBase,
  #[serde(alias="Third Base")]
  ThirdBase,
  #[serde(alias="Left Field")]
  LeftField,
  #[serde(alias="Right Field")]
  RightField,
}

#[derive(Deserialize, Debug)]
pub (crate) struct Teams {
  pub (crate) away: TeamData,
  pub (crate) home: TeamData,
}

#[derive(Deserialize, Debug)]
pub (crate) struct TeamData {
  pub (crate) team: Team,
  // #[serde(rename="player")]
  pub (crate) players: Vec<PlayerID>,
}

fn player_id_to_player (players: Vec<PlayerID>) -> Vec<Player> {
    players.into_iter()
      .map(|player| player.into())
      .collect()
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub (crate) struct PlayerID {
    pub (crate) person: Person,
    pub (crate) position: Position,
    pub (crate) batting_order: Option<String>,
    pub (crate) stats: Option<Stats>,
    pub (crate) all_positions: Option<Vec<Position>>,
}

// #[serde(from = "PlayerID")]
#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub struct Player {
  pub id: u32,
  pub position: Pos,
  pub batting_order: Option<u16>,
}

impl From<PlayerID> for Player {
  fn from(player: PlayerID) -> Player {

    let batting_order: Option<u16> = match player.batting_order {
      Some(order) => order.parse().ok(),
      None => None,
    };

    let sp: bool = match player.stats {
      Some (stats) => match stats.pitching.games_started {
        Some(games_started) => games_started == 1,
        None => false,
      },
      None => false,     
    };

    let pos = match player.all_positions {
      Some (position) => position[0].abbreviation, 
      None => player.position.abbreviation,
    };

    let position = match pos {
      Pos::Pitcher => {
        if sp {Pos::StartingPitcher} else {Pos::ReliefPitcher}
      }
      _ => pos, 
    };

    Player {
      id: player.person.id,
      position,
      batting_order,
    }

  }
}


#[derive(Deserialize, Debug)]
pub (crate) struct Person {
  pub (crate) id: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub (crate) struct Team {
  pub (crate) id: u32,
  //Future optimization to convert these to an enum, with yearly updates?
  pub (crate) name: String,
  pub (crate) team_code: String,
  pub (crate) location_name: Option<String>,
  pub (crate) league: LeagueIDName,
  pub (crate) sport: IDName,
  pub (crate) division: Option<IDName>,
  pub (crate) parent_org_id: Option<u32>,
  pub (crate) parent_org_name: Option<String>,

}

#[derive(Deserialize, Debug)]
pub (crate) struct LeagueIDName {
  pub (crate) id: Option<u32>,
  #[serde(alias="fullName")]
  pub (crate) name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub (crate) struct IDName {
  pub (crate) id: u32,
  #[serde(alias="fullName")]
  pub (crate) name: String,
}

#[derive(Deserialize, Debug)]
pub (crate) struct Info {
  pub (crate) label: String,
  pub (crate) value: Option<String>,
}

#[derive(Deserialize, Debug)]
pub (crate) struct Position {
  pub (crate) abbreviation: Pos,
}

#[derive(Deserialize, Serialize, Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum Pos {
  #[serde(rename="C")]
  Catcher,
  #[serde(rename="1B")]
  FirstBase,
  #[serde(rename="2B")]
  SecondBase,
  #[serde(rename="3B")]
  ThirdBase,
  #[serde(rename="SS")]
  ShortStop,
  #[serde(rename="LF")]
  LeftField,
  #[serde(rename="RF")]
  RightField,
  #[serde(rename="CF")]
  CenterField,
  #[serde(rename="P")]
  Pitcher,
  #[serde(rename="DH")]
  DesignatedHitter,
  StartingPitcher,
  ReliefPitcher,
  #[serde(other)]
  Bench,
}

impl From<Pos> for String {
  fn from (pos: Pos) -> String {
    match pos {
      Pos::Catcher => "C",
      Pos::FirstBase => "1B",
      Pos::SecondBase => "2B",
      Pos::ThirdBase => "3B",
      Pos::ShortStop => "SS",
      Pos::LeftField=> "LF",
      Pos::CenterField => "CF",
      Pos::RightField => "RF",
      Pos::Pitcher => "P",
      Pos::StartingPitcher => "SP",
      Pos::ReliefPitcher => "P",
      Pos::Bench => "Bench",
      Pos::DesignatedHitter => "DH"
    }.to_string()
  }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub (crate) struct Stats {
  pub (crate) pitching: PitchingStats,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub (crate) struct PitchingStats {
  pub (crate) games_started: Option<u8>,
}

