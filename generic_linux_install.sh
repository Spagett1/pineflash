#!/bin/sh
if command -v doas >> /dev/null; then
  root=doas
else 
  root=sudo
fi
cargo build --release


if [ "$(uname)" == "Darwin" ]; then
  curl -L "https://github.com/pine64/blisp/releases/download/v0.0.3/blisp-macos64-v0.0.3.zip" -o "blisp-macos64-v0.0.3.zip"
  unzip "blisp-macos64-v0.0.3.zip"
else 
  curl -L "https://github.com/pine64/blisp/releases/download/v0.0.3/blisp-linux64-v0.0.3.zip" -o "blisp-linux64-v0.0.3.zip"
  unzip "blisp-linux64-v0.0.3.zip"
fi

$root cp ./blisp/build/tools/blisp/blisp /usr/bin/blisp
$root cp ./Pineflash.desktop /usr/share/applications/Pineflash.desktop
$root cp ./assets/pine64logo.png /usr/share/pixmaps/pine64logo.png
$root cp ./target/release/pineflash /usr/bin/pineflash
