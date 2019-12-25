//! Utility crate used by other modules. Right now only includes an async get_all function.
//! Computation time to parse JSON is negligible, so we will simply make a big network request,
//! or rather a batch of requests, then process them.

use isahc::prelude::*;
use crate::*;
use std::env;

// use crate::error::*;

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
/// method for collecting an arbitrary number of files from the network.
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



