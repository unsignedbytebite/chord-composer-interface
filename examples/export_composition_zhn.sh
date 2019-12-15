#!/bin/bash

# Build
cd ../
cargo build --release --features zhn

# Run
clear
./target/release/chord-composer-interface export ./examples/example_composition.yaml