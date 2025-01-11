#!/bin/bash

# copy target to .local/bin

pkill waybar
cargo build --release
cp target/release/$(basename $(pwd)) ~/.local/bin/
hyprctl dispatch exec waybar

