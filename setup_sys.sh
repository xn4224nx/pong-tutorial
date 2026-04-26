#!/bin/bash
# https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md

sudo apt-get install g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0

# optional if you disabled the wayland feature
sudo apt-get install libwayland-dev libxkbcommon-dev
