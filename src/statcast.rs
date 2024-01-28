use std::fmt::format;

use reqwest::blocking::get as download;
use serde::{Serialize,Deserialize};
use csv::Reader;

const URL_BASE_01: &'static str = r#"https://baseballsavant.mlb.com/statcast_search/csv?hfPT=&hfAB=&hfGT=R%7C&hfPR=&hfZ=&hfStadium=&hfBBL=&hfNewZones=&hfPull=&hfC=&hfSea="#;
const URL_BASE_02: &'static str = r#"%7C&hfSit=&player_type=pitcher&hfOuts=&hfOpponent=&pitcher_throws=&batter_stands=&hfSA=&hfMo=&hfTeam=&home_road=&hfRO=&position=&hfInfield=&hfOutfield=&hfInn=&hfBBT=&hfFlag=&metric_1=&group_by=name&min_pitches=0&min_results=0&min_pas=0&sort_col=pitches&player_event_sort=api_p_release_speed&sort_order=desc&type=details&all=true"#;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct StatCast{
    pub rows: Vec<StatCastRow>,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct StatCastRow {
    pitch_type: PitchType,
    game_date: GameDate,
}

impl StatCast {
    pub fn download_day (game_date: GameDate) -> StatCast {
        let date_part = format!("&game_date_gt={}&game_date_lt={}", game_date.to_string(), game_date.to_string());
        let url = format!("{}{}{}{}", URL_BASE_01, game_date.year, URL_BASE_02, date_part);
        let csv_data = download(&url).expect("Didn't get a respone").text().expect("Couldn't download the CSV");
        
        let mut csv_reader = Reader::from_reader(csv_data.as_bytes());
        type CSVResult = Result <StatCastRow, csv::Error>;

        let rows: Vec<StatCastRow> = csv_reader.deserialize()
        .filter_map(|record: CSVResult| record.ok())
        .collect()
        ;
        
        Self { rows }
        
    }
    pub fn download_csv (game_date: String) {
        let date_part = format!("&game_date_gt={}&game_date_lt={}", game_date, game_date);
        let url = format!("{}{}{}{}cd", URL_BASE_01, "2022", URL_BASE_02, date_part);
        let file_name = format!("{}\\data\\{}.csv", std::env::current_dir().unwrap().display(), game_date.to_string());
        let mut file = std::fs::File::create(file_name).expect("couldn't create the file!");
        // let csv_data = download(&url).expect("Didn't get a response").bytes().expect("Couldn't download the CSV");
        let csv_data = download(&url).expect("Didn't get a response").copy_to(&mut file);
    }


}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub enum PitchType {
    CH,
    CS,
    CU,
    FC,
    FF,
    FS,
    KC,
    SI,
    SL,
    ST,
}


#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
#[serde(from = "&str")]
pub struct GameDate {
    year: u16,
    month: u8,
    day: u8,
}

impl From<&str> for GameDate {
    fn from(date: &str) -> Self {
        let date_parts : Vec<&str> =  date.split("-").collect();

        if date_parts.len() < 3 {panic!("Date not formatted correctly!")};

        Self { 
            year:  date_parts[0].parse().expect("Couldn't convert to a number"), 
            month: date_parts[1].parse().expect("Couldn't convert to a number"), 
            day:   date_parts[2].parse().expect("Couldn't convert to a number"), 
        }
    }
}

impl ToString for GameDate {
    fn to_string(&self) -> String {
        format!("{}-{}-{}", self.year, self.month, self.day)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn date_from_string() {
        let date_string = "2022-09-01";
        assert_eq!(GameDate {year: 2022, month: 09, day: 01}, GameDate::from(date_string));
    }
    #[test]
    #[should_panic]
    fn bad_date_panic() {
        let date_string = "2022\09\01";
        let _ = GameDate::from(date_string);
    }
}