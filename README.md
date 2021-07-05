# Code vs Zombies

A solution to the Code vs Zombies codingame.com challenge, written in Rust by raysplaceinspace

# How to run

One-time setup:
1. Install `cargo install cargo-merge`

Run:
1. `cargo watch --exec merge` (alternatively, just run `cargo merge` once)
2. Go to https://www.codingame.com/multiplayer/optimization/code-vs-zombies
3. Use the CodinGame Sync Google Chrome extension to upload the file to CodinGame

# Technical details

* Local search algorithm - performs random swaps/moves/adjustments to the pool of best solutions
* Solution represented as a series of milestones (e.g. move to a location, kill a particular zombie)
* Solutions are evaluated using a simulator
* Pool of best solutions consists of an ensemble of objective functions which are perturbations of the true objective function
