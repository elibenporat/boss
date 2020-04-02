//! Tools for grabbing venue data, both the basic data available from the API, as well as the svg files that will tell us where homeplate is.
//! The svg data is critical, since it will help us map the hit_data to actual coordinates on the field. It will also allow us to properly measure feet, since we'll hbe able to measure it against
//! the field's stated dimensions. Specifically, we'll look at the data points down the left and right field lines and see the distance in (x, y) that correlate to the HR distance down the lines.
//! This will allow us to convert the pixel distance to actual feet at the ballpark level.
//! 
//! We may also want to compute travel distance (flight time) from location to location, but this will be a feauture to add later.
//! 

use serde::{Deserialize, Serialize};
// use crate::utils::*;
use isahc::prelude::*;
use crate::cache;
use std::collections::BTreeSet;

/// Link to all the venues used by the MLB Stats API. "Hydrated" fields include the location, field dimension and
/// cross reference IDs that can be used to link to retrosheet.
const VENUES: &str = "https://statsapi.mlb.com/api/v1/venues/?hydrate=location,fieldInfo,timezone,xrefId";


pub fn test_venues () {
    let venues = isahc::get(VENUES).unwrap().text().unwrap();

    let venue_data: Venues = serde_json::from_str(&venues).unwrap();
 

    dbg!(&venue_data.venues[138]);

    let mut venues_x_y = cache::load_venue_x_y();

    let venues_cached: BTreeSet<u32> = venues_x_y
        .iter()
        .map(|venue| venue.id)
        .collect()
        ;


    let venues_x_y_new: Vec<VenueXY> = venue_data.venues
        .iter()
        .filter(|venue| !venues_cached.contains(&venue.id))
        .map(|venue| 
            {
                let id = venue.id;
                let (x,y) = get_svg(id);
                VenueXY {
                    id, x, y
                }
            }
        )
        .collect()
        ;

    venues_x_y.extend(venues_x_y_new);
    
    cache::cache_venue_x_y(&venues_x_y);

}

pub fn get_svg (id: u32) -> (Option<f32>, Option<f32>) {

    let link = format!("http://mlb.mlb.com/images/gameday/fields/svg/{}.svg", id);
    let svg_data = isahc::get(link).unwrap().text().unwrap();

    if svg_data.contains("Page Not Found") {
        return (None, None);
    }

    // The last <polyline> tag in the svg represents the baselines. The middle element is where the fair lines meet, which is the ideal
    // point to set the (x,y) coordinates

    let result = svg_data
        .split("<polyline").last().unwrap()
        .split("points=").nth(1).unwrap_or("")
        .split(" ").nth(1).unwrap_or("")
        .to_owned();

    if !result.contains(",") {return (None, None)};
    
    let split:Vec<&str> = result.split(",").collect();

    (split[0].parse::<f32>().ok(), split[1].parse::<f32>().ok()) 

}

// pub fn venue_svg() {

    
//     let svg_links: Vec<String> = (0 ..= 6_000)
//         .map(|id| format!("http://mlb.mlb.com/images/gameday/fields/svg/{}.svg", id))
//         .collect()
//         ;

//     let svg_data = stream_chunked(svg_links);

//     dbg!(&svg_data.len());

//     let _svgs: Vec<String> = svg_data
//         .into_iter()
//         .map(|svg| svg.unwrap())
//         .filter(|svg| !svg.contains("Page Not Found"))
//         .inspect(|svg| println!("{}", svg.split("<g id=").nth(1).unwrap_or(svg)))
//         .map(|svg| svg
//                 .split(r#"<g id="Base"#).nth(1).unwrap_or("")
//                 .split("<polygon").nth(1).unwrap_or("")
//                 .split("points=").nth(1).unwrap_or("")
//                 .split(" ").nth(2).unwrap_or("")
//                 .to_owned())
//         .inspect(|svg| println!("{}", svg))
//         .collect()
//         ;

// }

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub struct VenueXY {
    pub id: u32,
    pub x: Option<f32>,
    pub y: Option<f32>,
}


#[derive(Deserialize, Debug)]
pub (crate) struct Venues {
    pub (crate) venues: Vec<VenueDe>,
}

impl From<VenueDe> for Venue {
    fn from (v: VenueDe) -> Venue {
        
        let venue_retrosheet_id = v.xref_ids
                    .into_iter()
                    .filter(|id| id.xref_type == Some("retrosheet".to_string()))
                    .map(|id| id.xref_id.unwrap())
                    .nth(0)
                    .unwrap_or("".to_string())
                    ;
        
        let (venue_latitude, venue_longitude) = match v.location.default_coordinates {
            Some (loc) => (Some(loc.latitude), Some(loc.longitude)),
            _ => (None, None)
        };
        
        Venue {
            id: v.id,
            venue_name: v.name,
            venue_city: v.location.city,
            venue_state: v.location.state.unwrap_or("".to_string()),
            venue_state_abbr: v.location.state_abbrev.unwrap_or("".to_string()),
            venue_time_zone: v.time_zone.id,
            venue_time_zone_offset: v.time_zone.id.into(),
            venue_capacity: v.field_info.capacity,
            venue_surface: v.field_info.turf_type,
            venue_roof: v.field_info.roof_type,
            venue_retrosheet_id,
            venue_left_line: v.field_info.left_line,
            venue_left: v.field_info.left,
            venue_left_center: v.field_info.left_center,
            venue_center: v.field_info.center,
            venue_right_center: v.field_info.right_center,
            venue_right_line: v.field_info.right_line,
            venue_right: v.field_info.right,
            venue_latitude,
            venue_longitude,
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VenueData {
    pub year: u16,
    pub venue: Venue,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Venue {
    pub id: u32,
    pub venue_name: String,
    pub venue_city: String,
    pub venue_state: String,
    pub venue_state_abbr: String,
    pub venue_time_zone: TimeZone,
    pub venue_time_zone_offset: i8,
    pub venue_capacity: Option<u32>,
    pub venue_surface: Option<SurfaceType>,
    pub venue_roof: Option<RoofType>,
    pub venue_left_line: Option<u16>,
    pub venue_left: Option<u16>,
    pub venue_left_center: Option<u16>,
    pub venue_center: Option<u16>,
    pub venue_right_center: Option<u16>,
    pub venue_right: Option<u16>,
    pub venue_right_line: Option<u16>,
    pub venue_retrosheet_id: String,
    pub venue_latitude: Option<f32>,
    pub venue_longitude: Option<f32>,
}

impl Default for Venue {
    fn default() -> Self 
    {
        Venue {
            id: 401,
            venue_name: "".to_string(),
            venue_city: "".to_string(),
            venue_state: "".to_string(),
            venue_state_abbr: "".to_string(),
            venue_time_zone: TimeZone::EST,
            venue_time_zone_offset: 0,
            venue_capacity: None,
            venue_surface: None,
            venue_roof: None,
            venue_left_line: None,
            venue_left: None,
            venue_left_center: None,
            venue_center: None,
            venue_right_center: None,
            venue_right: None,
            venue_right_line: None,
            venue_retrosheet_id: "".to_string(),
            venue_latitude: None,
            venue_longitude: None,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all="camelCase")]
pub(crate) struct VenueDe {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) location: Location,
    pub(crate) time_zone: TimeZoneData,
    pub(crate) field_info: FieldInfo,
    #[serde(default="default_xref_ids")]
    pub(crate) xref_ids: Vec<XRefID>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all="camelCase")]
pub(crate) struct Location {
    pub(crate) city: String,
    pub(crate) state: Option<String>,
    pub(crate) state_abbrev: Option<String>,
    pub(crate) default_coordinates: Option<Coords>,
}

#[derive(Deserialize, Debug, Clone)]
pub (crate) struct Coords {
    pub(crate) latitude: f32,
    pub(crate) longitude: f32,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct TimeZoneData {
    pub(crate) id: TimeZone,
}

#[serde(rename_all="camelCase")]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct FieldInfo {
    pub(crate) capacity: Option<u32>,
    pub(crate) turf_type: Option<SurfaceType>,
    pub(crate) roof_type: Option<RoofType>,
    pub(crate) left_line: Option<u16>,
    pub(crate) left: Option<u16>,
    pub(crate) left_center: Option<u16>,
    pub(crate) center: Option<u16>,
    pub(crate) right_center: Option<u16>,
    pub(crate) right: Option<u16>,
    pub(crate) right_line: Option<u16>,
}

#[serde(rename_all="camelCase")]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct XRefID {
    pub(crate) xref_id: Option<String>,
    pub(crate) xref_type: Option<String>,
}

fn default_xref_ids() -> Vec<XRefID> {
        vec![]
}

// #[serde(field_identifier)]
#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub enum TimeZone {
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

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub enum SurfaceType {
    Artificial,
    Grass,
    Indoor,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub enum RoofType {
    Dome,
    Open,
    Retractable,
    Indoor,
}


