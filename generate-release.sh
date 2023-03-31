#!/bin/sh

ver=$(grep "^version" Cargo.toml | cut -d\" -f2)
mkdir ./release-build/
mkdir ./release-out/
cp ./PKGBUILD ./release-build/
cd ./release-build/
makepkg
cp pineflash-git*.tar.zst ../release-out/pineflash-$ver-x86.tar.zst
cd pkg/pineflash-git/
rm .*
zip -r pineflash_linux_x86_$ver.zip ./
mv ./pineflash_linux_x86_$ver.zip ../../../release-out/
rm -rf ./release-build
