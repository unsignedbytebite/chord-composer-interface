#!/bin/bash

# Build
cd ../
cargo build --release

# Run
clear
./target/release/chord-composer-interface play ./examples/example_composition.yaml --metronome --ticker-beat
