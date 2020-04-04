# BOSS | Baseball Open Source Software

A pure Rust baseball data aggregation and analytics library. For now supports easy data gathering from the MLB Stats API. Eventually, other sources such as RetroSheet and NCAA will be added.

All data retrieved by this application are subject to [copyright](http://gdx.mlb.com/components/copyright.txt) MLB Advanced Media, L.P.\
The author and this project are not affiliated with Major League Baseball.
 
BOSS is designed from the ground up to be extremely efficient. All text fields that can be converted to an enum have been carefully mapped, creating a highly compressed data set. The challenge with baseball data isn't the computational complexity of data gathering, it is the sheer size of the data set. One of BOSS' primary design goals is to be as efficient as possible.

BOSS leverages Rust's zero-cost asynchronous features and streams in data blazingly fast. This allows us to optimize the primary bottleneck - the dreaded network request.

Unlike all other packages the author is aware of, BOSS aims to provide a proverbial "batteries included" approach. This means that you can simply request the days/months/years and levels of baseball you want and get back a nicely packaged CSV with all the fields you care about. Boss' modular architecture will also enable you to drop down to the API itself and hack there. The data model is intentionally sparse, bringing in only the fields the author uses, in order to maximize efficiency. This may be limiting to some users. If it's an issue for you, open an issue.

## Documentation

https://docs.rs/boss

## Usage

```toml
[dependencies]
boss = "0.1"
```

## Similar Projects in Other Languages

**R**\
[baseballr](https://github.com/BillPetti/baseballr) by Bill Petti<br>
[pitchrx](https://github.com/cpsievert/pitchRx) by Carson Sievert

**Python**\
[MLB-StatsAPI](https://github.com/toddrob99/MLB-StatsAPI) by toddrob99<br>
[PyBall](https://github.com/bradleyhurley/PyBall) by Bradley Hurley

**JavaScript/Node.js**\
[mlb-stats-api](https://github.com/asbeane/mlb-stats-api) by Andy Beane

**Ruby**\
[MLBStatsAPI](https://github.com/Fustrate/mlb_stats_api) by Steven Hoffman

I'm sure there are more that I'm missing, please open an issue if there's a relevant project that should be added. With respect to the MLB Stats API, toddrob99's is probably the most complete of the bunch. Bill Petti's baseballr is probably the most complete package in terms of the sources it taps into. Toddrob99's project was incredibly helpful in finding a lot of the nitty gritty details. Thank you!

## Motivation

Building a baseball data engine in Rust will enable everyday fans to perform data-intensive workloads, as well as efficient data gathering. Ambitiously, aiming for a baseball data platform that will rival what MLB clubs have internally, from an analytics perspective. Clearly, MLB clubs will have access to more, and likely better, data, however, by leveraging Rust we should be able to build the most performant baseball data engine in the world.

This project is also a learning project for the author and will likely change a lot as the author better hones his Rust skills.

## Architecture

BOSS relies on the following crates for the bullk of its workload.

* **Isahc** handles all of the network requests to grab the JSON files. Isahc's powerful Futures support allowed for easy construction of Asynchronous requests and streams.
* **serde-json** combined with **SerDe** handle all the JSON parsing through declarative deserialization. We simply tell serde-json what structure to expect, point it to a file and the rest is handled magically.
* **csv** handles the bulk export of the data to the ubiquitious csv format. CSV is used since every BI tool can import it.

## API Structure

![API](https://github.com/elibenporat/boss/blob/boss/API.png)

## Features - Current and Planned

* Asynchronous out of the box. Player bios are memoized (cached) once they've been downloaded once, drastically reducing the number of network calls and the amount of time waiting on the network.
* Flattens out all the data and serializes to an easy to use CSV file that can be ported to Tableau or other
* The data pieces that take the most memory are the play descriptions, player names, etc. These are all compressed into hand-crafted enums, or kept in a hash table. This keeps the in-memory data set extremely small.
* Would love to build a Rust wrapper for the Tableau Hyper API, but don't how to do that... yet.

## Contributing

Contributions are certainly welcome once the basics are complete. I have never merged a pull request before, so you'll have to show me how :).

## Roadmap

* 0.1: Working library that exports a vector of pitches to CSV with a clean API
* 0.2: Error handling
* 0.3: Gameday XML data pull for extra data
* 0.4: Substitutions, RE288, Venue MetaData
* 0.5: Retrosheet integration
* 0.6: Fangraphs integration
* 0.7: Japanese/Korean stats
* 0.8: NCAA stats