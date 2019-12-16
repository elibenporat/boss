//! Proivides serialization for data pulled from the network. Data that has been pulled from the network, generally never needs to be pulled again.
//! For the vast majority of the meta-data, such as Venues, Teams and Players, these data don't change very much. This module will store the smaller data sets as JSON and 
//! use a custom, highly compact serialization for the pitch data.
//! 
//! This module is perhaps the most important as it will allow for easy incremental pulls. It will also serve as a demonstration for how efficicently a game can be stored.
//! The scraping modules will all need to be aware of the serialized files and have an Overwrite enum to allow for refreshing the data for whatever reason. All serialized data will
//! be stored in the "/local" folder with locations specified as consts
//! 
//! # Groups of Data
//! 
//! ## Venues
//! 
//! Venues have 3 layers of data. First, there is the unchanging aspect of each venue, such as the location. Second, we have per-season data which will vary for dimension as well as capacity and
//! possibly the venue name as well. Third, we have the SVG picture of the venue, from which we'll extract the (X,Y) coords of home plate. We'll also want to compute dimension from the play by play data
//! so that we can show the home run fence distance at 10 different points (0 degrees, 10 degrees ..= 90 degrees).
//! 
//! ## Players
//! Players have a "current" bio which is easy to query. They also have a lot of changes to their bio, specifically for their height and weight (mostly weight). Height and Weight are both good predictors of exit velo,
//! so if we're projecting prospects, we want to know what they weighed early in their career.