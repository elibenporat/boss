//! Main entry point for getting data. This module provides a simple API surface for getting data.
//! 
//! 


// We begin with the schedule. Getting the entire schedule can be quite time consuming, so we need to cache our data and only request year/level combinations that we haven't pulled already.
// Between 2005 and 2020 (inclusive) there are roughly 300K games in the database.

let schedule
