//! Main entry point for getting data. This module provides a simple API surface for getting data. You can always use any of the sub-modules to
//! get more granular access to the API, however, this will do all the work of piecing everything together.
//! 
//! 

use crate::boxscore::{BoxScoreDe, BoxScoreData, fix_boxscore};
use crate::cache::*;
use crate::coaches::{CoachData, Coaches, Roster};
use crate::feed_live::{FeedData,Feed};
use crate::metadata::{VecMetaDataInputs, MetaData};
use crate::play_by_play::Game;
use crate::players::{People, Player};
use crate::schedule::{YearRange, GameMetaData, Schedule, SeasonSportStatus, SeasonSportCache, SeasonStatus, AbstractGameState};
use crate::sports;
use crate::team::{TeamData, TeamJson};
use crate::utils::stream;
use crate::venues::{VenueXY, Venues, VenueData};
use crate::game::{Pitch, GameData};

use rayon::prelude::*;
use std::collections::{BTreeSet};
use isahc::prelude::*;
use serde::{Serialize, Deserialize};



// We begin with the schedule. Getting the entire schedule can be quite time consuming, so we need to cache our data and only request year/level combinations that we haven't pulled already.
// Between 2005 and 2020 (inclusive) there are roughly 300K games in the database.


pub fn get_everything() {
    let years = YearRange::from_range_inc(2005 ..= 2020);
    let sport_ids = sports::get_all_sport_ids();

    let meta = get_meta_data(years, sport_ids);
    let schedule = meta.schedule.clone();
    let meta_data = meta.into();

    for _ in 0 .. 50 {
        get_play_by_play(schedule.clone(), &meta_data);
    }
}

pub fn get_play_by_play (schedule: Vec<GameMetaData>, meta_data: &MetaData) {


    #[derive(Serialize, Deserialize)]
    struct GamesProcessed {
        good: BTreeSet<u32>,
        bad: BTreeSet<u32>,
    }

    let json = std::fs::read_to_string(r#"F:\Baseball\games_processed.json"#).unwrap();
    let games_processed: GamesProcessed = serde_json::from_str(&json).unwrap();

    // let mut stored_pbp = load_play_by_play();
    // let games_loaded: BTreeSet<u32> = stored_pbp.iter().map(|pitch| pitch.game_pk).collect();
    
    // println!("Loaded {} games", games_loaded.len());

    let mut good_games: BTreeSet<u32> = games_processed.good;
    let mut bad_games: BTreeSet<u32> = games_processed.bad;

    let pbp_urls: BTreeSet<(u32, String)> = schedule.iter()
        .filter (|game| game.game_status == AbstractGameState::Final)
        .filter (|game| !good_games.contains(&game.game_pk))
        .filter (|game| !bad_games.contains(&game.game_pk))
        .map(|game| (game.game_pk, format!("http://statsapi.mlb.com/api/v1/game/{}/playByPlay", game.game_pk)))
        .take(5_000)
        .collect()
        ;
        
    // we keep track of the requested games. Any that didn't return a result we'll package as a "bad" game
    let requested_games: BTreeSet<u32> = pbp_urls.iter().map(|game| game.0).collect();
    dbg!(requested_games.len());

    let http_client = isahc::HttpClient::new().unwrap();

    let result: Vec<Pitch> = pbp_urls.into_par_iter()
        // .inspect(|data| println!("{}", &data.1))
        .map (|data| (data.0, http_client.get(data.1).unwrap().text().unwrap()))
        .filter(|data| data.1.contains("allPlays"))
        .map (|data| {
            let pbp: Game = serde_json::from_str(&data.1).expect(&format!("Error with game_pk: {}", data.0));
            let game_data = GameData {
                pitch_data: pbp.all_plays,
                meta_data: &meta_data,
                game_pk: data.0,
            };
            let pitches: Vec<Pitch> = game_data.into();
            pitches
        })
        .flatten()
        .collect()
        ;
        
    
    let games_returned: BTreeSet<u32> = result.iter().map(|game|game.game_pk).collect();
    let games_missed: BTreeSet<u32> = requested_games.into_iter().filter(|game| !games_returned.contains(game)).collect();
    
    //They should equal the requested_games.len()
    dbg!(games_missed.len());
    dbg!(games_returned.len());

    good_games.extend(games_returned);
    bad_games.extend(games_missed);

    // stored_pbp.extend(result);

    // let good_games: BTreeSet<u32> = stored_pbp.iter().map(|game| game.game_pk).collect();


    let games_processed = GamesProcessed {
        good: good_games,
        bad: bad_games,
    };

    let json = serde_json::to_string(&games_processed).unwrap();
    std::fs::write(r#"F:\Baseball\games_processed.json"#, json).unwrap();

    println!("Writing to CSV...");
    crate::cache::append_play_by_play(&result);
    println!("Added {} records.", result.len());

}

/// Feed the get_data function a list of years and sport_ids and get back all the data for those combos. BOSS will cache anything it already has data for and try to fill in any missing pieces 
/// it doesn't have.
pub fn get_meta_data(years: Vec<u16>, sport_ids: Vec<u32>) -> VecMetaDataInputs {

    let schedule_data = get_schedule_data (years, sport_ids);
         
    let teams_data = get_team_data(&schedule_data);

    dbg!(teams_data.len());
    
    let boxscore_data = get_boxscore_data(&schedule_data);
    dbg! (boxscore_data.len());


    let coaches_data = get_coach_data(&schedule_data);
    dbg!(coaches_data.len());
                
    let player_data = get_player_data(&boxscore_data, &coaches_data);
    dbg!(player_data.len());
            
    let feed_live_data = get_feed_live_data(&schedule_data);
    dbg!(feed_live_data.len());

    let venue_x_y_data = get_venue_xy_data(&schedule_data);
    dbg!(venue_x_y_data.len());

    let venue_data = get_venue_data(&schedule_data);
    dbg!(venue_data.len());

    VecMetaDataInputs {
        schedule: schedule_data,
        boxscore: boxscore_data,
        venue: venue_data,
        venue_x_y: venue_x_y_data,
        coaches: coaches_data,
        feed_data: feed_live_data,
        teams: teams_data,
        players: player_data,
    }

    

}

fn get_team_data (sched: &Vec<GameMetaData>) -> Vec<TeamData> {

    let mut teams_cache = load_teams_data();

    let team_seaons_cached: BTreeSet<(u16, u32)>  = teams_cache.clone().iter()
        .map (|t| (t.year, t.team.sport_id))
        .collect()
        ;

    let team_seaons_needed: BTreeSet<(u16, u32)> = sched.iter()
        .map (|s| (s.game_date.year, s.sport_id))
        .collect()
        ;


    let team_urls: Vec<(u16, String)> = team_seaons_needed.iter()
        .filter(|team| !team_seaons_cached.contains(&(team.0, team.1)))
        .map (|team| (team.0, format!("http://statsapi.mlb.com/api/v1/teams/?season={}&sportId={}&hydrate=social", team.0, team.1)))
        // .take(1)
        // .inspect(|url| println!("{:?}", url))
        .collect()
        ;
    
    let http_client = isahc::HttpClient::new().unwrap();

    let json_data: Vec<(u16, String)> = team_urls.into_par_iter()
            .map (|url|  (url.0, http_client.get(url.1).unwrap().text().unwrap()))
            .filter (|json| json.1.contains("teams"))
            .collect();
            
    let new_team_data: Vec<TeamData> = json_data.into_par_iter()
        .map (|json| {
            let team: TeamJson = serde_json::from_str(&json.1).unwrap();
            let teams: Vec<TeamData> = team.teams.into_iter()
                .map(|team| TeamData {
                    year: json.0,
                    team: team.clone().into(),
                })
                .collect()
                ;
            teams
        })
        .flatten()
        .collect()
        ;

    teams_cache.extend (new_team_data);
    cache_teams_data(&teams_cache);
    teams_cache
}

fn get_player_data (boxscore: &Vec<BoxScoreData>, coaches: &Vec<CoachData>) -> Vec<Player> {

    let mut players_cache = load_player_data();

    let mut players_needed: BTreeSet<u32> = boxscore.iter()
        .map(|b| 
            {
                let mut players: Vec<crate::boxscore::Player> = b.boxscore_data.home_players.clone();
                players.extend(b.boxscore_data.away_players.clone());
                players
            })
        .flatten()
        .map(|player| player.id)
        .collect()
        ;
    
    let umps_needed: BTreeSet<u32> = boxscore.iter()
        .filter_map( |b| b.boxscore_data.hp_umpire_id)
        .collect()
        ;

    let coaches_needed: BTreeSet<u32> = coaches.iter()
        .map (|c| vec![
                c.home_coaches.batting_coach,
                c.home_coaches.pitching_coach,
                c.home_coaches.manager,
                c.away_coaches.batting_coach,
                c.away_coaches.pitching_coach,
                c.away_coaches.manager,
            ])
        .flatten()
        .filter_map(|c| c)
        .collect()
        ;

    players_needed.extend(umps_needed);
    players_needed.extend(coaches_needed);
    

    // dbg!(players_needed.len());

    let players_cached: BTreeSet<u32> = players_cache.clone().into_iter()
            .map(|player| player.id)
            .collect();
    
    let player_urls: BTreeSet<String> = players_needed.iter()
            .filter (|player| !players_cached.contains(&player))
            .map(|player| format!("http://statsapi.mlb.com/api/v1/people/{}?hydrate=xrefId,draft,transactions,awards,education",player))
            // .inspect(|url| println!("{}", url))
            .take(1_000)
            .collect()
            ;

    if player_urls.len() == 0 {return players_cache};

    let http_client = isahc::HttpClient::new().unwrap();

    let json_data: Vec<String> = player_urls.into_par_iter()
            .map (|url|  http_client.get(url).unwrap().text().unwrap())
            .collect();

    let new_player_data: Vec<crate::players::Player> = json_data.into_par_iter()
        .filter (|json| json.contains("people"))
        .map (|json| {
            let player: People = serde_json::from_str(&json).unwrap();
            player.people[0].clone().into()
        })
        .collect()
        ;
    players_cache.extend(new_player_data);
    cache_player_data(&players_cache);
    players_cache
}

fn get_venue_data (schedule_data: &Vec<GameMetaData>) -> Vec<VenueData> {
    
    let mut venue_cache = load_venue();

    let venues_cached: BTreeSet<(u32, u16)> = venue_cache.clone().into_iter()
        .map(|venue_season| (venue_season.venue.id, venue_season.year))
        .collect();

    // If the year specific data isn't there, try the generic version
    let venue_urls: BTreeSet<(u16,String, String)> = schedule_data.iter()
        .filter(|game| !venues_cached.contains(&(game.game_venue_id, game.game_date.year)))
        .map ( |game| (
            game.game_date.year, 
            format!("https://statsapi.mlb.com/api/v1/venues/{}/?hydrate=location,fieldInfo,timezone,xrefId&season={}", game.game_venue_id, game.game_date.year),
            format!("https://statsapi.mlb.com/api/v1/venues/{}/?hydrate=location,fieldInfo,timezone,xrefId", game.game_venue_id)
        ))
        //VenueID 526 never has any data, so IGNORE IT
        .filter(|game| !game.1.contains("/526"))
        .collect();

    if venue_urls.len() == 0 {return venue_cache};
    dbg!(&venue_urls);
    
    let http_client = isahc::HttpClient::new()
    .unwrap();

    let json_data: Vec<(u16, String, String)> = venue_urls.into_par_iter()
        .map(|url| (url.0, http_client.get(url.1).unwrap().text().unwrap(), http_client.get(url.2).unwrap().text().unwrap()))
        .collect()
        ;
    
    let new_venue_data: Vec<VenueData> = json_data.into_par_iter()
        .map (|data| {
            if data.1.contains("venues") {
                (data.0, data.1)
            }
            else {
                (data.0, data.2)
            }
        })
        .filter (|json| json.1.contains("venues"))
        .map (|json| {
            let venue: Venues = serde_json::from_str(&json.1).unwrap();
            VenueData {
                year: json.0,
                venue: venue.venues[0].clone().into(),
            }
        })
        .collect()
        ;
    
    venue_cache.extend(new_venue_data);
    cache_venue(&venue_cache);
    venue_cache       

    
}

fn get_coach_data (schedule_data: &Vec<GameMetaData>) -> Vec<CoachData> {


    let mut coaches_cache = load_coach_data();
    let games_cached: BTreeSet<u32> = coaches_cache.clone().into_iter()
        .map (|coaches| coaches.game_pk)
        .collect()
        ;

    let coach_urls: Vec<(u32, String, String)> = schedule_data.iter()
        .filter(|game| !games_cached.contains(&game.game_pk) && game.game_status == AbstractGameState::Final)
        .map(|game| (game.game_pk, game.coaches_home_url.clone(), game.coaches_away_url.clone()))
        .take(20_000)
        .collect()
        ;
    
    if coach_urls.len() == 0 {return coaches_cache};

    let http_client = isahc::HttpClient::new()
        .unwrap();
    
    let json_data: Vec<(u32,String, String)> = coach_urls.into_par_iter()
    .map(|url| (url.0, http_client.get(url.1).unwrap().text().unwrap(),  http_client.get(url.2).unwrap().text().unwrap()))
    .collect()
    ;    

    let new_coach_data: Vec<CoachData> = json_data.into_par_iter()
        .map(|json| 
            {
            let home_coaches: Result<Roster, serde_json::Error> = serde_json::from_str(&json.1);
            let away_coaches: Result<Roster, serde_json::Error> = serde_json::from_str(&json.2);
            (json.0, home_coaches, away_coaches)
            }
        )
        // .inspect(|game| if game.1.is_err() {println!("Game pk: {} \n Error: {:?} ", game.0, game.1)})
        // .filter(|coaches| coaches.1.is_ok() && coaches.2.is_ok())
        .map (|coaches| {
            let no_coaches = Coaches {
                batting_coach: None,
                pitching_coach: None,
                manager: None,
             };

            let home_coaches = match coaches.1 {
                Ok (coaches) => coaches.into(),
                _ => no_coaches.clone(),
            };
            let away_coaches = match coaches.2 {
                Ok (coaches) => coaches.into(),
                _ => no_coaches.clone(),
            };

            CoachData {
                game_pk: coaches.0,
                home_coaches,
                away_coaches,
        }})
        .collect();

        coaches_cache.extend(new_coach_data);
        cache_coach_data(&coaches_cache);
        coaches_cache

}

fn get_venue_xy_data (schedule_data: &Vec<GameMetaData>) -> Vec<VenueXY> {

    let x_y_venues: BTreeSet<u32> = schedule_data.iter()
            .map(|game| game.game_venue_id)
            .collect();
    
    let mut venues_x_y = load_venue_x_y();

    let venues_cached: BTreeSet<u32> = venues_x_y
        .iter()
        .map(|venue| venue.id)
        .collect()
        ;


    let venues_x_y_new: Vec<VenueXY> = x_y_venues
        .iter()
        .filter(|venue| !venues_cached.contains(&venue))
        .map(|id| 
            {
                let id = *id;
                let (x,y) = crate::venues::get_svg(id);
                VenueXY {
                    id, x, y
                }
            }
        )
        .collect()
        ;
    
    if venues_x_y_new.len() == 0 {return venues_x_y};

    venues_x_y.extend(venues_x_y_new);
    
    cache_venue_x_y(&venues_x_y);

    venues_x_y
}

fn get_boxscore_data (schedule_data: &Vec<GameMetaData>) -> Vec<BoxScoreData> {

    let mut boxscore_cache = load_boxscore_data();
    let games_cached: BTreeSet<u32> = boxscore_cache.clone().into_iter()
        .map (|boxscore| boxscore.game_pk)
        .collect()
        ;


    let error_games = vec![431736, 597930, 220889, 431736, 259009, 577313];

    let boxscore_urls: Vec<(u32, String)> = schedule_data.iter()
        .filter(|game| !games_cached.contains(&game.game_pk) && game.game_status == AbstractGameState::Final)
        .filter(|game| !error_games.contains(&game.game_pk))
        .map(|game| (game.game_pk, game.game_url_boxscore.clone()))
        .take(500)
        // .take(0)
        .collect()
        ;

    if boxscore_urls.len() == 0 {return  boxscore_cache};
    

    let http_client = isahc::HttpClient::new().unwrap();

    // let start_time = std::time::Instant::now();
    // let http_client = isahc::HttpClient::builder()
    //     .max_connections(1)
    //     .build()
    //     .unwrap();

    let json_data: Vec<(u32,String)> = boxscore_urls.into_par_iter()
        .map(|url| (url.0, http_client.get(url.1).unwrap().text().unwrap()))
        .collect()
        ;

    // println!("Took {} seconds to pull the boxscore data", start_time.elapsed().as_secs());

    let new_boxscore_data: Vec<BoxScoreData> = json_data.into_par_iter()
    .map(|json| 
            {
            let fixed_box = fix_boxscore(&json.1);
            let boxscore_json: Result<BoxScoreDe, serde_json::Error> = serde_json::from_str(&fixed_box);
            (json.0, boxscore_json)
            }
        )
        .inspect(|game| if game.1.is_err() {println!("Game pk: {} \n Error: {:?} ", game.0, game.1)})
        .filter(|boxscore| boxscore.1.is_ok())
        .map(|boxscore| BoxScoreData {
            game_pk: boxscore.0,    
            boxscore_data: boxscore.1.unwrap().into(),
        })
        .collect()
        ;
    boxscore_cache.extend(new_boxscore_data);
    cache_boxscore_data(&boxscore_cache);
    boxscore_cache

}

fn get_feed_live_data (schedule_data: &Vec<GameMetaData>) -> Vec<FeedData> {

    let mut feed_live_cache = load_feed_live_data();
    let games_cached: BTreeSet<u32> = feed_live_cache.clone().into_iter()
            .map (|game| game.game_pk)
            .collect()
            ;

    let bad_games = vec![
        431736, 220889, 597930, 307853, 259009, 383397, 383405, 383409, 383411,
    ];

    let feed_urls: Vec<String> = schedule_data.iter()
            .filter(|game| !games_cached.contains(&game.game_pk) && game.game_status == AbstractGameState::Final)
            .filter(|game| !bad_games.contains(&game.game_pk))
            .map(|game| game.game_url_feed_live.clone())
            .take(500)
            .collect()
            ;

    if feed_urls.len() == 0 {return feed_live_cache};

    // dbg!(&feed_urls);

    // let start_time = std::time::Instant::now();

    // Split up the pull into chunks of 4 urls and then spawn a separate thread for each chunk.
    // Each chunk will then do an asynchronous request so that we get multi-threaded asynchronous
    // network requests. I don't know the optimal chunk_size to use here.
    let json_data: Vec<Result<String, std::io::Error>> = feed_urls.into_par_iter()
        .chunks(20)
        .map(|url| stream(url))
        .flatten()
        // .inspect(|game| if game.is_err() {println!("Error: {:?}", game)})
        .collect()
        ;

    // println!("Took {} seconds to pull the feed live data", start_time.elapsed().as_secs());

    let new_feed_live_data: Vec<FeedData> = json_data.into_par_iter()
        .map(|json| 
            {
                let json_data = json.unwrap_or("".to_string());
                let feed_data: Result<Feed, serde_json::Error> = serde_json::from_str(&json_data);
                feed_data
            }
        )
        // .inspect(|game| if game.is_err() {println!("Error: {:?}", game)})
        .filter(|feed_data| feed_data.is_ok())
        .map(|feed_data| feed_data.unwrap().into())
        .collect()
        ;
    
    feed_live_cache.extend(new_feed_live_data);
    cache_feed_live_data(&feed_live_cache);

    feed_live_cache

}

fn get_schedule_data (years: Vec<u16>, sport_ids: Vec<u32>) -> Vec<GameMetaData> {

    let schedule_cache = load_schedule();  
    
    // Figure out which seasons need to be re-pulled. Any season/sport_id combination
    // that has any items that aren't "Final" should be re-pulled. We first map all the
    // schedule data into a set, then build a second set with our filter rule. Currently,
    // does not keep track of Empty seasons, so will always re-try to pull all season/sport_ids
    // combinatations that it hasn't marked as "Complete"
    let season_sports_status: SeasonSportStatus =
        schedule_cache
            .clone()
            .into_iter()
            .map(|sched| (sched.game_date.year, sched.sport_id, sched.game_status))
            .collect()
            ;
    
    let season_sports: SeasonSportCache =
    schedule_cache
        .clone()
        .into_iter()
        .map(|sched| ((sched.game_date.year, sched.sport_id), SeasonStatus::status(sched.game_date.year, sched.sport_id, &season_sports_status)))
        .collect()
        ;

    let sched = Schedule::get_data(years, sport_ids, &season_sports);
    let games: Vec<GameMetaData> = sched.games.into_iter()
                                        .map(|game| game.into())
                                        .collect()
                                        ;

    let mut schedule_data: Vec<GameMetaData> = schedule_cache.into_iter()
                    .filter(|sched| season_sports.get(&(sched.game_date.year, sched.sport_id)) == Some(&SeasonStatus::Complete))
                    .collect::<Vec<GameMetaData>>()
                    ;

    schedule_data.extend(games);
    cache_schedule(&schedule_data);
    
    dbg!(&schedule_data.len());
    schedule_data

}
