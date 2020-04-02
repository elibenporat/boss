//! Utility crate used by other modules. Inlcludes streaming functions as well as simple date math.
//! 
//! 
//! 


use isahc::prelude::*;
use crate::*;
use std::env;
// use std::time;
use serde::{Serialize, Deserialize};

// use crate::error::*;

/// Date Struct used by the crate. We'll build our own date handling rather than bring in an extra depedency. The date maths we need to do are
/// fairly simple (calculating Batter/Pitcher age) and should help keep our dependency tree as small as possible.
/// To calculate age, we'll simply do year-year + (month-month)/12 + (day-day)/365.25. We'll be off by a little on the day side,
/// but it doesn't really matter if we track a batter as 24.21 years old instead of 24.25 years old.
/// 
/// We ignore the time from the gameDate string as we'll get more accurate info from the boxscore, where we want the "Frist Pitch"
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
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


//TODO - ADD ERROR HANDLING
pub fn get_directory() -> String {
    let path = env::current_dir().unwrap();
    path.to_str().unwrap().into()
}


/// Splits the network request into CHUNK_SIZE items. Only use if the regular stream function throws a network
/// timeout error. Will perform slightly worse than the stream function since it waits for each CHUNK_SIZE to come in.
pub fn stream_chunked (urls: Vec<String>) -> Vec<Result<String, std::io::Error>> {

    let mut stream_result: Vec<Result<String, std::io::Error>> = Vec::with_capacity(urls.len());

    for chunk in urls.chunks(CHUNK_SIZE) {
        let result = stream(chunk.to_owned());
        stream_result.extend(result);
    }

    stream_result
}


/// Stream will send out a bunch of requests and collect them as they come in. This is an extremely efficient
/// method for collecting an arbitrary number of files from the network. If there is a risk that too many requests
/// will cause a time_out, use stream_chunked instead
pub fn stream (urls: Vec<String>) -> Vec<Result<String, std::io::Error>> {
   
    let resp_stream = futures::stream::FuturesUnordered::new();
    
    for url in urls {
        resp_stream.push (
            async {
                isahc::get_async(url).await?.text_async().await
            }
        )
    };

    futures::executor::block_on_stream(resp_stream).collect()


}



