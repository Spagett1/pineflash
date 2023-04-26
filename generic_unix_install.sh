#!/bin/bash
if command -v doas >> /dev/null; then
  root=doas
else 
  root=sudo
fi

cargo build --release

if [ "$(uname)" == "Darwin" ]; then
  curl -L "https://github.com/pine64/blisp/releases/download/v0.0.3/blisp-macos64-v0.0.3.zip" -o "blisp-macos64-v0.0.3.zip"
  unzip "blisp-macos64-v0.0.3.zip"
  chmod +x ./blisp
  $root cp ./blisp /usr/local/bin/blisp
  $root chmod +x /usr/local/bin/blisp
  $root cp ./target/release/pineflash /usr/local/bin/
elif [ "$(uname -m)" == "x86_64" ]; then
  curl -L "https://github.com/pine64/blisp/releases/download/v0.0.3/blisp-linux64-v0.0.3.zip" -o "blisp-linux64-v0.0.3.zip"
  unzip "blisp-linux64-v0.0.3.zip"
  $root cp ./blisp /usr/local/bin/blisp
  $root chmod +x /usr/local/bin/blisp
  $root cp ./assets/Pineflash.desktop /usr/share/applications/Pineflash.desktop
  $root cp ./assets/pine64logo.png /usr/share/pixmaps/pine64logo.png
  $root cp ./target/release/pineflash /usr/bin/pineflash
else 
  git clone --recursive "https://github.com/pine64/blisp.git"
  cd blisp
  git submodule update --init --recursive
  mkdir build && cd build
  cmake -DBLISP_BUILD_CLI=ON ..
  cmake --build .
  $root cp ./blisp/build/tools/blisp/blisp /usr/local/bin/blisp
  $root chmod +x /usr/local/bin/blisp
  cd ../..
  $root cp ./assets/Pineflash.desktop /usr/share/applications/Pineflash.desktop
  $root cp ./assets/pine64logo.png /usr/share/pixmaps/pine64logo.png
  $root cp ./target/release/pineflash /usr/bin/pineflash
fi


