#!/bin/sh
# Remove old versions
rm pineflash*{.deb,.rpm,.exe,.tar*,.AppImage} 2> /dev/null
ver=$(grep "^version " Cargo.toml | cut -d\" -f2)
# files=target/release/pineflash=/usr/bin/pineflash assets/Pineflash.desktop=/usr/share/applications/Pineflash.desktop assets/pine64logo.png=/usr/share/pixmaps/pine64logo.png LICENSE=/usr/share/licenses/pineflash/LICENSE
arch="x86_64"
cargo clean
cargo build --target x86_64-pc-windows-gnu --release
cargo appimage --features=appimage
mv target/appimage/pineflash.AppImage "./pineflash-$ver-$arch.AppImage"

# Generate windows release
# Update version number 
sed -i "s/#define MyAppVersion.*/#define MyAppVersion \"$ver\"/g" ./Package_Windows_Release.iss
wine /home/spagett/.wine/drive_c/Program\ Files\ \(x86\)/Inno\ Setup\ 6/ISCC.exe ./Package_Windows_Release.iss
mv PineFlash_Installer.exe pineflash-$ver-win64.exe

# Cleans for new environment
cargo clean
# Generate rpm release
distrobox enter --name fedora-dev -- cargo build --release && fpm -s dir -t rpm \
  --name pineflash \
  --license gpl2 \
  --version $ver \
  --architecture $arch \
  --depends polkit \
  --depends dfu-util \
  -p "pineflash-fedora-$ver-$arch.rpm" \
  --description "flashing tool for pinecil soldering irons." \
  --url "https://github.com/spagett1/pineflash" \
  --maintainer "spagett <laar@tutanota.com>" \
  target/release/pineflash=/usr/bin/pineflash assets/Pineflash.desktop=/usr/share/applications/Pineflash.desktop assets/pine64logo.png=/usr/share/pixmaps/pine64logo.png LICENSE=/usr/share/licenses/pineflash/LICENSE tools/linux/blisp=/usr/bin/blisp

# Cleans for new environment
cargo clean

# Generate rpm release for rhel
distrobox enter --name rhel-dev -- cargo build --release && fpm -s dir -t rpm \
  --name pineflash \
  --license gpl2 \
  --version $ver \
  --architecture $arch \
  --depends polkit \
  --depends dfu-util \
  -p "pineflash-rhel9-$ver-$arch.rpm" \
  --description "flashing tool for pinecil soldering irons." \
  --url "https://github.com/spagett1/pineflash" \
  --maintainer "spagett <laar@tutanota.com>" \
  target/release/pineflash=/usr/bin/pineflash assets/Pineflash.desktop=/usr/share/applications/Pineflash.desktop assets/pine64logo.png=/usr/share/pixmaps/pine64logo.png LICENSE=/usr/share/licenses/pineflash/LICENSE tools/linux/blisp=/usr/bin/blisp


# Cleans for new environment
cargo clean

distrobox enter --name debian-dev -- cargo build --release && fpm -s dir -t deb \
  --name pineflash \
  --license gpl2 \
  --version $ver \
  --architecture $arch \
  --depends policykit-1 \
  --depends dfu-util \
  --depends libxkbcommon0 \
  --description "Flashing tool for pinecil soldering irons." \
  --url "https://github.com/Spagett1/PineFlash" \
  --maintainer "Spagett <laar@tutanota.com>" \
  target/release/pineflash=/usr/bin/pineflash assets/Pineflash.desktop=/usr/share/applications/Pineflash.desktop assets/pine64logo.png=/usr/share/pixmaps/pine64logo.png LICENSE=/usr/share/licenses/pineflash/LICENSE tools/linux/blisp=/usr/bin/blisp

# Cleans for new environment
cargo clean

# # Generate arch release
distrobox enter --name arch-dev -- cargo build --release && fpm -s dir -t pacman \
  --name pineflash \
  --license gpl2 \
  --version $ver \
  --architecture $arch \
  --depends polkit \
  --depends libxkbcommon \
  --depends dfu-util \
  --description "Flashing tool for pinecil soldering irons." \
  --url "https://github.com/Spagett1/PineFlash" \
  --maintainer "Spagett <laar@tutanota.com>" \
  target/release/pineflash=/usr/bin/pineflash assets/Pineflash.desktop=/usr/share/applications/Pineflash.desktop assets/pine64logo.png=/usr/share/pixmaps/pine64logo.png LICENSE=/usr/share/licenses/pineflash/LICENSE tools/linux/blisp=/usr/bin/blisp
# # Generate generic release
# fpm -s dir -t tar \
#   --name pineflash \
#   --license gpl2 \
#   --version $ver \
#   --architecture $arch \
#   --depends polkit \
#   --description "Flashing tool for pinecil soldering irons." \
#   --url "https://github.com/Spagett1/PineFlash" \
#   --maintainer "Spagett <laar@tutanota.com>" \
#   target/release/pineflash=/usr/bin/pineflash assets/Pineflash.desktop=/usr/share/applications/Pineflash.desktop assets/pine64logo.png=/usr/share/pixmaps/pine64logo.png LICENSE=/usr/share/licenses/pineflash/LICENSE
# mv pineflash.tar pineflash-$ver-1-x86_64.tar
# xz pineflash-$ver-1-x86_64.tar
#
