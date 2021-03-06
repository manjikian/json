#!/usr/bin/env sh

clear
rustc --crate-type lib src/json.rs
rustdoc --extern json=libjson.rlib --test src/json.rs
rm libjson.rlib