//! Tools for grabbing venue data, both the basic data available from the API, as well as the svg files that will tell us where homeplate is.
//! The svg data is critical, since it will help us map the hit_data to actual coordinates on the field. It will also allow us to properly measure feet, since we'll hbe able to measure it agains
//! the field's stated dimensions. Specifically, we'll look at the data points down the left and right field lines and see the distance in (x, y) that correlate to the HR distance down the lines.
//! This will allow us to convert the pixel distance to actual feet at the ballpark level.
//! 
//! We may also want to compute travel distance (flight time) from location to location, but this will be a feauture to add later.
//! 

use serde::{Deserialize};
use crate::utils::*;
use isahc::prelude::*;

/// Link to all the venues used by the MLB Stats API. "Hydrated" fields include the location, field dimension and
/// cross reference IDs that can be used to link to retrosheet.
const VENUES: &str = "https://statsapi.mlb.com/api/v1/venues/?hydrate=location,fieldInfo,timezone,xrefId";


pub fn test_venues () {
    let venues = isahc::get(VENUES).unwrap().text().unwrap();
    let venue_data: Venues = serde_json::from_str(&venues).unwrap();


    for venue in venue_data.venues {
        let svg = get_svg(venue.id);
        if svg == "".to_string() {
            dbg! (venue.id);
        }
    }

}

// This is a horrible, ugly way to do it, but it works
fn get_svg (id: u32) -> String {

    let link = format!("http://mlb.mlb.com/images/gameday/fields/svg/{}.svg", id);
    let svg_data = isahc::get(link).unwrap().text().unwrap();

    if svg_data.contains("Page Not Found") {
        return "No SVG Data".to_string();
    }

    let first_attempt = svg_data
        .split(r#"<g id="Base"#).nth(1).unwrap_or("")
        .split("<polygon").nth(1).unwrap_or("")
        .split("points=").nth(1).unwrap_or("")
        .split(" ").nth(2).unwrap_or("")
        .to_owned()
        ;
    
    let second_attempt = svg_data
        .split(r#"<polygon id="home"#).nth(1).unwrap_or("")
        .split("points=").nth(1).unwrap_or("")
        .split(" ").nth(2).unwrap_or("")
        .to_owned()
        ;
    
    let third_attempt = svg_data
        .split("<polygon fill=\"#FFFFFF\" stroke=\"#FFFFFF\"").nth(1).unwrap_or("")
        .split("points=").nth(1).unwrap_or("")
        .split(" ").nth(2).unwrap_or("")
        .to_owned()
        ;
    
    
    if first_attempt != "".to_string() {
        first_attempt
    }
    else if second_attempt != "".to_string() {
        second_attempt
    }
    else {
        third_attempt
    }

}

pub fn venue_svg() {

    
    let svg_links: Vec<String> = (0 ..= 6_000)
        .map(|id| format!("http://mlb.mlb.com/images/gameday/fields/svg/{}.svg", id))
        .collect()
        ;

    let svg_data = stream_chunked(svg_links);

    dbg!(&svg_data.len());

    let _svgs: Vec<String> = svg_data
        .into_iter()
        .map(|svg| svg.unwrap())
        .filter(|svg| !svg.contains("Page Not Found"))
        .inspect(|svg| println!("{}", svg.split("<g id=").nth(1).unwrap_or(svg)))
        .map(|svg| svg
                .split(r#"<g id="Base"#).nth(1).unwrap_or("")
                .split("<polygon").nth(1).unwrap_or("")
                .split("points=").nth(1).unwrap_or("")
                .split(" ").nth(2).unwrap_or("")
                .to_owned())
        .inspect(|svg| println!("{}", svg))
        .collect()
        ;

    

  
    // dbg!(&venue_data.venues[0..15]);

}

#[derive(Deserialize, Debug)]
pub struct Venues {
    venues: Vec<Venue>,
}

impl From<VenueDe> for Venue {
    fn from (v: VenueDe) -> Venue {
        Venue {
            id: v.id,
            venue_name: v.name,
            venue_city: v.location.city,
            venue_state: v.location.state,
            venue_state_abbr: v.location.state_abbrev,
            venue_time_zone: v.time_zone.id,
            venue_time_zone_offset: v.time_zone.id.into(),
            venue_capacity: v.field_info.capacity,
            venue_surface: v.field_info.turf_type,
            venue_roof: v.field_info.roof_type,
        }
    }
}

impl From<TimeZone> for i8 {
    fn from (t: TimeZone) -> i8 {
        match t {
            TimeZone::HST => -10,
            TimeZone::PST => -8,
            TimeZone::MST => -7,
            TimeZone::CST => -6,
            TimeZone::EST => -5,
            TimeZone::VET | TimeZone::AST => -4,
            TimeZone::ASIA => 9,
            TimeZone::AUSTRALIA => 11,
            TimeZone::EUROPE => 0,
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(from="VenueDe")]
pub struct Venue {
    id: u32,
    venue_name: String,
    venue_city: String,
    venue_state: Option<String>,
    venue_state_abbr: Option<String>,
    venue_time_zone: TimeZone,
    venue_time_zone_offset: i8,
    venue_capacity: Option<u32>,
    venue_surface: Option<SurfaceType>,
    venue_roof: Option<RoofType>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
struct VenueDe {
    id: u32,
    name: String,
    location: Location,
    time_zone: TimeZoneData,
    field_info: FieldInfo,
    #[serde(default="default_xref_ids")]
    xref_ids: Vec<XRefID>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
struct Location {
    city: String,
    state: Option<String>,
    state_abbrev: Option<String>,
    default_coordinates: Option<Coords>,
}

#[derive(Deserialize, Debug)]
struct Coords {
    latitude: f32,
    longitude: f32,
}

#[derive(Deserialize, Debug)]
struct TimeZoneData {
    id: TimeZone,
}

#[serde(rename_all="camelCase")]
#[derive(Deserialize, Debug)]
struct FieldInfo {
    capacity: Option<u32>,
    turf_type: Option<SurfaceType>,
    roof_type: Option<RoofType>,
}

#[serde(rename_all="camelCase")]
#[derive(Deserialize, Debug)]
struct XRefID {
    xref_id: String,
    xref_type: String,
}

fn default_xref_ids() -> Vec<XRefID> {
        vec![]
}

#[serde(field_identifier)]
#[derive(Deserialize, Debug, Copy, Clone)]
enum TimeZone {
    /// ### GMT -10
    /// * Pacific/Honolulu
    #[serde(alias="Pacific/Honolulu")]
    HST,    
    /// ### GMT -8
    /// * America/Los_Angeles
    /// * America/Vancouver
    /// * America/Tijuana
    #[serde(alias="America/Los_Angeles", alias="America/Vancouver", alias="America/Tijuana")]
    PST, 
    /// ### GMT -7 Includes the following time zones:
    /// * America/Hermosillo
    /// * America/Denver
    /// * America/Phoenix
    /// * America/Boise
    /// * America/Edmonton
    #[serde(alias="America/Hermosillo", alias="America/Denver", alias="America/Phoenix", alias="America/Boise", alias="America/Edmonton")]
    MST,
    /// ### GMT -6
    /// * America/Chicago
    /// * America/Monterrey
    /// * America/Cancun
    /// * America/Mexico_City
    /// * America/Winnipeg
    /// * America/Merida
    /// * America/Mazatlan
    /// * America/Havana
    /// * America/Matamoros
    /// * America/Guatemala
    #[serde(alias="America/Chicago", alias="America/Monterrey", alias="America/Cancun", alias="America/Mexico_City", 
            alias="America/Winnipeg", alias="America/Merida", alias="America/Mazatlan", alias = "America/Havana",
            alias="America/Matamoros", alias="America/Guatemala")]
    CST,
    /// ### GMT -5
    /// * America/New_York
    /// * America/Panama
    /// * America/Toronto
    /// * America/Detroit
    /// * America/Kentucky/Louisville
    /// * America/Indiana/Indianapolis
    #[serde(alias="America/New_York", alias="America/Panama", alias="America/Toronto", alias="America/Detroit", alias="America/Kentucky/Louisville", alias="America/Indiana/Indianapolis")]
    EST,
    /// ### GMT -4
    /// * America/Caracas
    #[serde(alias="America/Caracas")]
    VET,
    /// ### GMT -4
    /// * America/Puerto_Rico 
    /// * America/Santo_Domingo
    /// * America/Halifax
    #[serde(alias="America/Puerto_Rico", alias="America/Santo_Domingo", alias="America/Halifax")]
    AST,
    /// ### GMT +9
    /// All of China/Taiwan/Japan lumped together here
    /// * Asia/Tokyo
    /// * Asia/Taipei
    /// * Asia/Seoul
    /// * Asia/Shanghai
    #[serde(alias="Asia/Tokyo", alias="Asia/Taipei", alias="Asia/Seoul", alias="Asia/Shanghai")]
    ASIA, 
    /// ### GMT +11
    /// Grouping all the australia locations together, even though this is slightly innacurate because it doesn't really matter.
    /// * Australia/Sydney
    /// * Australia/Perth
    /// * Australia/Brisbane
    /// * Australia/Melbourne
    /// * Australia/Adelaide
    #[serde(alias="Australia/Sydney", alias="Australia/Perth", alias="Australia/Brisbane", alias="Australia/Melbourne", alias="Australia/Adelaide")]
    AUSTRALIA,
    /// ### GMT ??
    /// All of Europe lumped into here
    /// * Europe/Helsinki
    /// * Europe/Stockholm
    /// * Europe/London
    /// * Europe/Moscow
    /// * Europe/Rome
    /// * Europe/Berlin
    /// * Asia/Jerusalem
    /// * Europe/Amsterdam
    /// * Europe/Prague
    #[serde( alias="Europe/Helsinki", alias="Europe/Stockholm", alias="Europe/London", alias="Europe/Moscow", alias="Europe/Rome",
             alias= "Europe/Berlin", alias="Asia/Jerusalem", alias="Europe/Amsterdam", alias="Europe/Prague")]
    EUROPE
}

#[derive(Deserialize, Debug)]
enum SurfaceType {
    Artificial,
    Grass,
    Indoor,
}

#[derive(Deserialize, Debug)]
enum RoofType {
    Dome,
    Open,
    Retractable,
    Indoor,
}


