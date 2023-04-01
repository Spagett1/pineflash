<a href="https://github.com/River-Mochi/PineFlash#pineflash"><img src="https://hits.seeyoufarm.com/api/count/incr/badge.svg?url=https%3A%2F%2Fgithub.com%2FSpagett1%2FPineFlash&count_bg=%23187BC0&title_bg=%23555555&icon=&icon_color=%23E7E7E7&title=hits&edge_flat=true"/></a>
[![GitHub all downloads](https://img.shields.io/github/downloads/spagett1/pineflash/total?color=187BC0&style=flat-square)](https://github.com/Spagett1/PineFlash/releases/tag/0.3.0)
[![GitHub release (latest by date)](https://img.shields.io/github/v/release/spagett1/pineflash?color=187BC0&style=flat-square)](https://github.com/Spagett1/PineFlash/releases/tag/0.3.0)

# PineFlash

<img src="https://user-images.githubusercontent.com/77225642/192753666-1a0e2bf4-b5ec-4e35-ba31-aae9043e04b9.png" align="right" width="425" style="float:left">
A GUI tool to flash IronOS to the Pinecil V1, V2 and future other pine products.  

## Features
* Auto-fetch the newest stable release of IronOS firmware.
* Allows manual installs of beta versions using a browse to file feature.
* Selectable options: pick the iron type, pick the firmware version and download it (current path is /tmp).

<br clear="both" />

## Supported Devices 
 | System  |<img width="17" src="https://cdn.simpleicons.org/Linux/000000" /> Linux  | <img width="15" src="https://cdn.simpleicons.org/Apple" /> MacOS|  <img width="15" src="https://cdn.simpleicons.org/Windows11/000000" /> Windows|
 | :-----: | :-----: | :-----: | :-----: |
 | Pinecil V1 |<img width="18" src="https://cdn.simpleicons.org/cachet/187BC0" />|<img width="18" src="https://cdn.simpleicons.org/cachet/187BC0" />| wip  |
 | Pinecil V2 | <img width="18" src="https://cdn.simpleicons.org/cachet/187BC0" />   | <img width="18" src="https://cdn.simpleicons.org/cachet/187BC0" />  |  wip  |
 <br>
<br clear="both" />



## Dependancies
```
polkit
dfu-util - Pinecil V1 only
```
### Disclaimer: This does not currently work on wayland.

## :computer: Connect Pinecil to a PC

* To connect Pinecil V1 or V2 to the computer to do the firmware update, connect one end of a USB cable to the PC. 
* Then, hold down the `[-]` button before plugging the usb-c cable to the back of Pinecil.
* Keep holding the `[-]` for ~10 seconds more before releasing the button. If you correctly entered flashing mode, the screen will be black/empty.


## Install
Go over to https://github.com/Laar3/PineFlash/releases, intructions can be found there.
Options:
Pre-made Binaries: pineflash zip or tar releases are the easiest options to install

## :runner: Run 
Linux: `pkexec env DISPLAY=$DISPLAY XAUTHORITY=$XAUTHORITY pineflash`

MacOS: simply running `pineflash` will work fine as it doesn't need root privledges. 
<br>

# Build from code

## Build Dependancies

```
git
rust
cmake
polkit
gtk3 (arch based distros) / libgtk-3-dev (debian based distros)
dfu-util # For pinecil v1 support. 
```

## Build Linux dev
1. install all dependencies
2. extract the source code tar.gz from the newest Assets [in releases](https://github.com/Spagett1/PineFlash/releases/)
3. run the `generic_linux_install.sh` file which will build and install Pineflash.

## Arch based distro's
1. All dependancies will be handled by the PKGBUILD
2. You can use the PKGBUILD which will handle everything for you.
3. Just run `makepkg -si` in the main directory to build and install it.

## Manual build
1. download the git submodules 
```
git submodule update --init --recursive
```
2. build blisp which is needed for pinecil V2 support 
```
cd blisp
mkdir build
cd build
cmake -DBLISP_BUILD_CLI=ON ..
cmake --build .
sudo mv ./tools/blisp/blisp /usr/bin/ #Or some other global path.
```
 :bookmark: Important: Don't forget to add blisp to your path

then to build pineflash itself
```
cargo build --release
```
The resulting binary will be in `target/release/pineflash`, this can be moved into your path (`/usr/bin/pineflash`) or just run as a portable executable.

Then copy the Pineflash.desktop file to `/usr/share/applications` and copy `assets/pine64logo.png` to `/usr/share/pixmaps` for the shortcut to show up in launchers.

Note, on linux root permissions are needed for dfu-util, in order to solve this you need to run the program with the following command if running from the terminal `pkexec env DISPLAY=$DISPLAY XAUTHORITY=$XAUTHORITY pineflash`.

This is already in the .desktop file so dont worry about that if you just use it.

 

## Todo

- [x] Some sort of toast while stuff is downloading, flashing, etc.
- [x] V2 support 
- [ ] Windows support
- [ ] In app instructions/images for getting the pinecil ready to flash
## Feel like supporting me?
Well you can buy me a coffee (or rather tea bags or something since i dont drink coffee ;))

<a href="https://www.buymeacoffee.com/spagett" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/default-orange.png" alt="Buy Me A Coffee" height="41" width="174"></a>
## References
- [Blisp](https://github.com/pine64/blisp) - Backend for flashing Pinecil V2
- [Dfu-util](https://dfu-util.sourceforge.net/) - Backend for flashing Pinecil V1
- [Pinecil](https://wiki.pine64.org/wiki/Pinecil) - The Pinecil Wiki page
- [IronOS](https://github.com/Ralim/IronOS) - The firmware running on this soldering iron
