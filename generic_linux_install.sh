#!/bin/sh
if command -v doas >> /dev/null; then
  root=doas
else 
  root=sudo
fi
git submodule update --init --recursive
cargo build --release
$root cp ./Pineflash.desktop /usr/share/applications/Pineflash.desktop
$root cp ./assets/pine64logo.png /usr/share/pixmaps/pine64logo.png
$root cp ./target/release/pineflash /usr/bin/pineflash
