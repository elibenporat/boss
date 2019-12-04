///Various helpers - not sure yet if I'll need this module, mostly just here for experimenting
/// 
/// 

use futures::stream::futures_unordered::*;
use isahc::prelude::*;
use std::time::Instant;
use rayon::prelude::*;

// const NUMBERS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

// fn to_str(num: T, len: u8) -> &'static str
// where T:Unsigned {
//     let mut 
//     for _ in 0..len {
        
//     }
// }


pub fn stream () {

    // let schedule_stream = FuturesUnordered::new();
    let urls: Vec<String> = vec![ 
        "http://statsapi.mlb.com/api/v1//schedule?sportId=1&startDate=01/01/2012&endDate=12/31/2012".to_string(),
        "http://statsapi.mlb.com/api/v1//schedule?sportId=1&startDate=01/01/2013&endDate=12/31/2013".to_string(),
        "http://statsapi.mlb.com/api/v1//schedule?sportId=1&startDate=01/01/2014&endDate=12/31/2014".to_string(),
        "http://statsapi.mlb.com/api/v1//schedule?sportId=1&startDate=01/01/2015&endDate=12/31/2015".to_string(),
        "http://statsapi.mlb.com/api/v1//schedule?sportId=1&startDate=01/01/2016&endDate=12/31/2016".to_string(),
        "http://statsapi.mlb.com/api/v1//schedule?sportId=1&startDate=01/01/2017&endDate=12/31/2017".to_string(),
        "http://statsapi.mlb.com/api/v1//schedule?sportId=11&startDate=01/01/2012&endDate=12/31/2012".to_string(),
        "http://statsapi.mlb.com/api/v1//schedule?sportId=11&startDate=01/01/2013&endDate=12/31/2013".to_string(),
        "http://statsapi.mlb.com/api/v1//schedule?sportId=11&startDate=01/01/2014&endDate=12/31/2014".to_string(),
        "http://statsapi.mlb.com/api/v1//schedule?sportId=11&startDate=01/01/2015&endDate=12/31/2015".to_string(),
        "http://statsapi.mlb.com/api/v1//schedule?sportId=11&startDate=01/01/2016&endDate=12/31/2016".to_string(),
        "http://statsapi.mlb.com/api/v1//schedule?sportId=11&startDate=01/01/2017&endDate=12/31/2017".to_string(),
    ];

    let resp_stream = FuturesUnordered::new();
    let mut par_resp_stream: Vec<_> = vec![];
   
    let par_async_time = Instant::now();
    for _ in 0..10 {
        let par_resp = FuturesUnordered::new();
        for url in urls.clone() {
            par_resp.push(async {isahc::get_async(url).await.unwrap().text().unwrap()});
        }
        par_resp_stream.push(par_resp);
    }

    let _results: Vec<Vec<String>> = par_resp_stream.into_par_iter()
                    .map(|stream| futures::executor::block_on_stream(stream).collect() )
                    .collect()
                    ;
    
    println!("Finished parallel async pull in: {:?}", par_async_time.elapsed());

    let async_time = Instant::now();
    for _ in 0..10 {
        for url in urls.clone() {
            resp_stream.push(async {isahc::get_async(url).await.unwrap().text().unwrap()});

        }
    }
    let _results: Vec<String> = futures::executor::block_on_stream(resp_stream).collect();
    
    
    println!("Finished async pull in: {:?}", async_time.elapsed());
    

    let sync_time = Instant::now();

    for _ in 0..10 {
        for url in urls.clone(){
            let _result = isahc::get(url).unwrap().text().unwrap();
        }
    }
     println!("Finished sync pull in: {:?}", sync_time.elapsed());


    // let games:Vec<crate::schedule::Games> = results
    //     .iter()
    //     .map(|json| {
    //         let sched: crate::schedule::ScheduleDe = serde_json::from_str(&json).unwrap();
    //         let games: crate::schedule::Games = sched.into();
    //         games
    //     })
    //     .collect()
    //     ;

    // for game in games {
    //     dbg!(&game[0].game_date_year);
    // }

    
    // for url in urls {
    //     let resp_fut = isahc::get_async(url).unwrap().text().unwrap();
    //     schedule_stream.push()
    // }

}