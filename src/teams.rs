//! API Bindings for pulling in team data.
//! 
//! Data should be pulled in season-by-season to ensure the correct parent_team_org_id is pulled. 
//! Data include xref_ids for FaceBook, Twitter, Instagram and StubHub, which could allow us to see fanbases and average ticket prices.
//! 
//! We pull in the Spanish versions of MLB team's twitter and facebook pages. If the Spanish version == English version, we toss it out, since there is no unique page.
//! TODO: Build functionality to scrape current popularity of these pages as well as ticket prices (StubHub and TicketMaster).
//! 
//! API link: http://statsapi.mlb.com/api/v1/teams/?hydrate=hydrations,xrefId&season={}
//! 
//! 


use serde::{Serialize, Deserialize}

#[derive(Serialize, Deserialize, Debug)]
#[serde(from="TeamDeserialize")]
pub struct Team {
    id: u32,
    name: String,
    location: String,
    division_id: u32,
    division_name: String,
    league_id: u32,
    league_name: String,
    sport_id: u32,
    sport_name: u32,
    parent_org_id: u32,
    parent_org_name: String,
    twitter_id: String,
    twitter_es_id: String,
    retrosheet_id: String,
    instagram_id: String,
    facebook_id: String,
    facebook_es_id: String,
    stub_hub_id: String,
    ticket_master_id: String,
    season: u16,
    first_year_of_play: u16,
}

#[derive(Deserialize, Debug)]
struct TeamDeserialize {
    id: u32,
    name: String,
    location: String,

}