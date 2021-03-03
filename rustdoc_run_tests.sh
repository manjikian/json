#!/usr/bin/env sh

clear
rustc --crate-type lib src/lib.rs
rustdoc --extern lib=liblib.rlib --test src/lib.rs
rm liblib.rlib