#!/bin/sh
ver=$(grep "^version" Cargo.toml | cut -d\" -f2)
files="target/release/pineflash=/usr/bin/pineflash assets/Pineflash.desktop=/usr/share/applications/Pineflash.desktop assets/pine64logo.png=/usr/share/pixmaps/pine64logo.png LICENSE=/usr/share/licenses/pineflash/LICENSE"
arch="x86_64"
cargo build --release
cargo build --target x86_64-pc-windows-gnu --release

# Generate windows release
# Update version number 
sed -i "s/#define MyAppVersion.*/#define MyAppVersion \"$ver\"/g" ./Package_Windows_Release.iss
wine /home/spagett/.wine/drive_c/Program\ Files\ \(x86\)/Inno\ Setup\ 6/ISCC.exe ./Package_Windows_Release.iss
mv PineFlash_Installer.exe pineflash-$ver_win64.exe


# Generate rpm release
fpm -s dir -t rpm \
  --name pineflash \
  --license gpl2 \
  --version $ver \
  --architecture $arch \
  --depends polkit \
  --description "Flashing tool for pinecil soldering irons." \
  --url "https://github.com/Spagett1/PineFlash" \
  --maintainer "Spagett <laar@tutanota.com>" \
  $files
# Generate deb release
fpm -s dir -t deb \
  --name pineflash \
  --license gpl2 \
  --version $ver \
  --architecture $arch \
  --depends polkit \
  --description "Flashing tool for pinecil soldering irons." \
  --url "https://github.com/Spagett1/PineFlash" \
  --maintainer "Spagett <laar@tutanota.com>" \
  $files
# Generate arch release
fpm -s dir -t pacman \
  --name pineflash \
  --license gpl2 \
  --version $ver \
  --architecture $arch \
  --depends polkit \
  --description "Flashing tool for pinecil soldering irons." \
  --url "https://github.com/Spagett1/PineFlash" \
  --maintainer "Spagett <laar@tutanota.com>" \
  $files
# Generate generic release
fpm -s dir -t tar \
  --name pineflash \
  --license gpl2 \
  --version $ver \
  --architecture $arch \
  --depends polkit \
  --description "Flashing tool for pinecil soldering irons." \
  --url "https://github.com/Spagett1/PineFlash" \
  --maintainer "Spagett <laar@tutanota.com>" \
  $files
mv pineflash.tar pineflash-$ver-1-x86_64.tar
xz pineflash-$ver-1-x86_64.tar

