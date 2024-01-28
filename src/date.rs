/// Date Struct used by the crate. We'll build our own date handling rather than bring in an extra depedency. The date maths we need to do are
/// fairly simple (calculating Batter/Pitcher age) and should help keep our dependency tree as small as possible.
/// To calculate age, we'll simply do year-year + (month-month)/12 + (day-day)/365.25. We'll be off by a little on the day side,
/// but it doesn't really matter if we track a batter as 24.21 years old instead of 24.25 years old.
/// 
/// We ignore the time from the gameDate string as we'll get more accurate info from the boxscore, where we want the "Frist Pitch"

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl ToString for Date {
    fn to_string(&self) -> String {
        format!("{}-{}-{}", self.year, self.month, self.day)
    }
}

/// Get the difference between two dates in units of years. Does a rough approximation only. This is primarily used for computing
/// the age of the batter and does not need to be super precise. This saves us from having to import a library to do proper date
/// math. This may change in the future.
impl std::ops::Sub for Date {
    type Output = f32;

    fn sub(self, other:Date) -> f32 {
        
        let year_diff = self.year as f32 - other.year as f32;
        let month_diff = self.month as f32 - other.month as f32;
        let day_diff = self.day as f32 - other.day as f32;

        year_diff + (month_diff / 12f32 ) + (day_diff / 365.25f32)
    
    }

}

impl From<String> for Date {
    fn from(value: String) -> Self {
        let date_parts: Vec<u16> = value.split("-").map(|p| p.parse().unwrap()).collect();
        Self {
            year: date_parts[0],
            month: date_parts[1] as u8,
            day: date_parts[2] as u8,
        }
    }
}