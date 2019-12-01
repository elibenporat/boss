BOSS | Baseball Open Source Software
===

A pure Rust baseball data aggregation and analytics library. Supports data aggregation from a number of sources
including the MLB stats API, MLB gameday files. Eventually, other sources such as RetroSheet and NCAA will be added.
 
BOSS is designed from the ground up to be extremely efficient. All text fields that can be converted to an enum have been
carefully mapped. The challenge with baseball data isn't the computational complexity of data gathering, it is the sheer
size of the data set. One of BOSS' primary design goals is to be as efficient as possible.

### Documentation

TODO

### Usage

```toml
[dependencies]
boss = "0.1"
```
### Prior Art

[baseballr](https://github.com/BillPetti/baseballr) by Bill Petti<br>
[pitchrx](https://github.com/cpsievert/pitchRx) by Carson Sievert

### Motivation

Building a baseball data engine in Rust will enable everyday fans to perform data-intensive workloads, as well as efficient data gathering. Ambitiously, aiming for a baseball data platform that will rival what MLB clubs have internally, from an analytics perspective. Clearly, MLB clubs will have access to more, and likely better, data, however, by leveraging Rust we should be able to build the most performant baseball data engine in the world.

This project is also a learning project for the author and should change a lot as the author better hones his Rust skills.

### Architecture
BOSS relies on three crates for the bullk of its workload. <br>
* <b>Isahc</b> handles all of the network requests to grab the JSON files. Isahc's powerful Futures support allowed for easy construction of Asynchronous requests.
* <b>serde-json</b> combined with <b>SerDe</b> handle all the JSON parsing through declarative deserialization. We simply tell serde-json what structure to expect, point it to a file and the rest is handled magically.

Rayon is used to add parallelism. At some point, I'm hoping this evolves into Async Parallel Generators (or something like that) where Rayon is aware of all the yield points in any of its iterations so it can bounce around as needed.

### Features - Current and Planned
* Parallel out of the box. Player bios are memoized (cached) once they've been downloaded once, drastically reducing the number of network calls.
* Captures historical player weight info (only for XML version, might add to JSON version later)
* Flattens out all the data and serializes to an easy to use CSV file that can be ported to Tableau or other
* Would love to build a Rust wrapper for the Tableay Hyper API, but don't how to do that... yet.
* The data pieces that take the most memory are the play descriptions, however these are very repetitive and should be highly compressible. If we ever build a non-flat materialization, we'll probably want to compress the descriptions.


### Roadmap

* Tools to gather data from the GameDay xml files, for all levels of affiliated baseball (mostly complete, files are deprecated by MLB)
* Tools to gather statcast data, as well as calculation for spin efficiency (these are from the JSON files, we'll)
* Incoporate the Rust retrosheet parser and try to align the data to the GameDay and StatCast data sets. Hopefully will be able to use the existing code base
* Export flattened (denormalized) games to CSV (Ideally, there should be an option to split out into 2 files, one for metadata, one for play-by-play, rather than just one big flat file)