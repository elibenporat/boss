===

A pure Rust baseball data aggregation and analytics library. 
Currently only does aggregation from the MLB GameDay XML files, as well as processing base out game state.

### Documentation

TODO

### Usage

```toml
[dependencies]
boss = "0.2"
```
### Prior Art

[baseballr](https://github.com/BillPetti/baseballr) by Bill Petti<br>
[pitchrx](https://github.com/cpsievert/pitchRx) by Carson Sievert

Non-scientific benchmarks show the Rust version performs about 4X as fast as the R version, though this is difficult to measure precisely since the vast majority of the time is spent waiting on the network. Typical CPU usage is negligible using BOSS (peaked at less than 1% on my PC), though this may vary depending on your hardware.

### Motivation

Building a baseball data engine in Rust will enable everyday fans to perform data-intensive workloads, as well as efficient data gathering. Ambitiously, aiming for a baseball data platform that will rival what MLB clubs have internally, from an analytics perspective. Clearly, MLB clubs will have access to more, and likely better, data.

This project is also a learning project for the author and should change a lot as the author better hones his Rust skills.

### Architecture
BOSS relies on three crates for the bullk of its workload. <br>
* <b>Isahc</b> handles all of the network requests to grab the JSON files. Isahc's powerful Futures support allowed for easy construction of Asynchronous requests.
* <b>serde-json</b> combined with <b>SerDe</b> handle all the JSON parsing through declarative deserialization. We simply tell serde-json what structure to expect, point it to a file and the rest is handled magically.

Rayon is used to add parallelism. At some point, I'm hoping this evolves into Async Parallel Generators (or something like that) where Rayon is aware of all the yield points in any of its iterations so it can bounce around as needed.

### Features
* Parallel out of the box. Player bios are memoized (cached) once they've been downloaded once, drastically reducing the number of network calls.
* Captures historical player weight info
* Flattens out all the data and serializes to an easy to use CSV file.

### Roadmap

* Tools to gather data from the GameDay xml files, for all levels of affiliated baseball
* Tools to gather statcast data, as well as calculation for spin efficiency
* Incoporate the Rust retrosheet parser and try to align the data to the GameDay and StatCast data sets. Hopefully will be able to use the existing code base
* Export flattened (denormalized) games to CSV (Ideally, there should be an option to split out into 2 files, one for metadata, one for play-by-play, or just one big flat file)