BOSS | Baseball Open Source Software
===

A pure Rust baseball data aggregation and analytics library. For now supports easy data gathering from the MLB Stats API. Eventually, other sources such as RetroSheet and NCAA will be added.

All data retrieved by this application are subject to [copyright](http://gdx.mlb.com/components/copyright.txt) MLB Advanced Media, L.P. <br>
The author and this project are not affiliated with Major League Baseball.
 
BOSS is designed from the ground up to be extremely efficient. All text fields that can be converted to an enum have been carefully mapped, creating a highly compressed data set. The challenge with baseball data isn't the computational complexity of data gathering, it is the sheer size of the data set. One of BOSS' primary design goals is to be as efficient as possible.

BOSS leverages Rust's zero-cost asynchronous features and streams in data blazingly fast. This allows us to optimize the primary bottleneck - the dreaded network request.

Unlike all other packages the author is aware of, BOSS aims to provide a proverbial "batteries included" approach. This means that you can simply request the days/months/years and levels of baseball you want and get back a nicely packaged CSV with all the fields you care about. Boss' modular architecture will also enable you to drop down to the API itself and hack there. The data model is intentionally sparse, bringing in only the fields the author uses, in order to maximize efficiency. This may be limiting to some users. If it's an issue for you, open an issue.

### Documentation

https://docs.rs/boss

### Usage

```toml
[dependencies]
boss = "0.1"
```
### Similar Projects in Other Languages

<b>R</b><br>
[baseballr](https://github.com/BillPetti/baseballr) by Bill Petti<br>
[pitchrx](https://github.com/cpsievert/pitchRx) by Carson Sievert

<b>Python</b><br>
[MLB-StatsAPI](https://github.com/toddrob99/MLB-StatsAPI) by toddrob99<br>
[PyBall](https://github.com/bradleyhurley/PyBall) by Bradley Hurley

<b>JavaScript/Node.js</b><br>
[mlb-stats-api](https://github.com/asbeane/mlb-stats-api) by Andy Beane

<b>Ruby</b><br>
[MLBStatsAPI](https://github.com/Fustrate/mlb_stats_api) by Steven Hoffman

I'm sure there are more that I'm missing, please open an issue if there's a relevant project that should be added. With respect to the MLB Stats API, toddrob99's is probably the most complete of the bunch. Bill Petti's baseballr is probably the most complete package in terms of the sources it taps into. Toddrob99's project was incredibly helpful in finding a lot of the nitty gritty details. Thank you!

### Motivation

Building a baseball data engine in Rust will enable everyday fans to perform data-intensive workloads, as well as efficient data gathering. Ambitiously, aiming for a baseball data platform that will rival what MLB clubs have internally, from an analytics perspective. Clearly, MLB clubs will have access to more, and likely better, data, however, by leveraging Rust we should be able to build the most performant baseball data engine in the world.

This project is also a learning project for the author and will likely change a lot as the author better hones his Rust skills.

### Architecture
BOSS relies on three crates for the bullk of its workload. <br>
* <b>Isahc</b> handles all of the network requests to grab the JSON files. Isahc's powerful Futures support allowed for easy construction of Asynchronous requests and streams.
* <b>serde-json</b> combined with <b>SerDe</b> handle all the JSON parsing through declarative deserialization. We simply tell serde-json what structure to expect, point it to a file and the rest is handled magically.

### Features - Current and Planned
* Asynchronous out of the box. Player bios are memoized (cached) once they've been downloaded once, drastically reducing the number of network calls and the amount of time waiting on the network.
* Flattens out all the data and serializes to an easy to use CSV file that can be ported to Tableau or other
* The data pieces that take the most memory are the play descriptions, player names, etc. These are all compressed into hand-crafted enums, or kept in a hash table. This keeps the in-memory data set extremely small.
* Would love to build a Rust wrapper for the Tableay Hyper API, but don't how to do that... yet.


### Roadmap

* Legacy tools to gather data from the deprecated gameday xml files
* Complete rust bindings to the mlb.com stats api
* Incorporate the Rust retrosheet parser and try to align the data to theStatCast data set. Hopefully we will be able to use the existing code base We're capturing the xref_ids so this should be possible.
* Export flattened (denormalized) games to CSV (Ideally, there should be an option to split out into 2 files, one for metadata, one for play-by-play, rather than just one big flat file)