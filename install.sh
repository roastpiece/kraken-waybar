#!/bin/bash

# copy target to .local/bin

cargo build --release
cp target/release/$(basename $(pwd)) ~/.local/bin/

