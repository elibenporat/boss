# ROADMAP

## Current Features

BOSS will stream, parse and de-normalize all useful data from the MLB Stats API. These include a host of metadata, as well as all the relevant pitch level data.
Currently, the only supported output mode is CSV.

## 0.10 Release

A 0.10 release of BOSS entails wrapping a clean API around what's been built. Currently, all the data gathering bits are hardcoded in. This needs to be converted
to an abstraction that library users can use.

## Missing Analytics Features

* **RE288:** Currently, we use the 2018 RE288 tables from Tom Tango. We need built in logic that builds them for each league/year combination. The current values should
be directionally correct, but they don't sum up to zero, which is inelegant at the very least.
