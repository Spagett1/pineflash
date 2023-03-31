![Hit Counter](https://img.shields.io/endpoint?color=blue&style=flat-square&url=https%3A%2F%2Fhits.dwyl.com%2Fspagett1%2Fpineflash.json)
![GitHub all downloads](https://img.shields.io/github/downloads/spagett1/pineflash/total?color=blue&style=flat-square)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/spagett1/pineflash?style=flat-square)

# PineFlash


<img src="https://user-images.githubusercontent.com/77225642/192753666-1a0e2bf4-b5ec-4e35-ba31-aae9043e04b9.png" align="right" width="450" style="float:left">
A GUI tool to flash IronOS to the Pinecil V1, V2 and future other pine products.  

## Features
* Auto-fetch the newest stable release of IronOS firmware.
* Allows manual installs of beta versions using a browse to file feature.
* Selectable options: pick the iron type, pick the firmware version and download it (current path is /tmp).
* This currently only supports unix-like operating systems (macos and linux), windows support is in the works.

<br clear="both" />

## Supported Devices 
 | System  |<img width="17" src="https://cdn.simpleicons.org/Linux/000000" /> Linux  | <img width="15" src="https://cdn.simpleicons.org/Apple" /> MacOS|  <img width="15" src="https://cdn.simpleicons.org/Windows11/000000" /> Windows|
 | :-----: | :-----: | :-----: | :-----: |
 | Pinecil V1 |<img width="18" src="https://cdn.simpleicons.org/cachet/5791ac" />|<img width="18" src="https://cdn.simpleicons.org/cachet/5791ac" />|   |
 | Pinecil V2 | <img width="18" src="https://cdn.simpleicons.org/cachet/5791ac" />   | <img width="18" src="https://cdn.simpleicons.org/cachet/5791ac" />  |    |
 <br>
<br clear="both" />



## Dependancies
```
dfu-util - Pinecil V1 only
polkit
```

## Connect Pinecil to a PC

* To connect Pinecil V1 or V2 to the computer to do the firmware update, connect one end of a USB cable to the PC. 
* Then, hold down the `[-]` button before plugging the usb-c cable to the back of Pinecil.
* Keep holding the `[-]` for ~10 seconds more before releasing the button. If you correctly entered flashing mode, the screen will be black/empty.



### Disclaimer: This does not currently work on wayland.
## Install
Go over to https://github.com/Laar3/PineFlash/releases, intructions can be found there.

## Run 
 Linux: `pkexec env DISPLAY=$DISPLAY XAUTHORITY=$XAUTHORITY pineflash`

MacOS: simply running `pineflash` will work fine as it doesn't need root privledges. 
<br>

## Build from code

### Build Dependancies
```
git
rust
gtk3 (arch based distros) / libgtk-3-dev (debian based distros)
```

## Linux
After extracting the source code tar.gz, then run the `generic_linux_install.sh` file which will build and install Pineflash.

### Arch based distro's
All dependancies will be handled by the PKGBUILD
You can use the PKGBUILD which will handle everything for you.
Just run `makepkg -si` in the main directory to build and install it.


## Manual build
First download the git submodules 
```
git submodule update --init --recursive
```
after this build blisp which is needed for pinecil V2 support 
```
cd blisp
mkdir build
cd build
cmake -DBLISP_BUILD_CLI=ON ..
cmake --build .
sudo mv ./tools/blisp/blisp /usr/bin/ #Or some other global path.
```
**Important: Don't forget to add blisp to your path**
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
- [ ] In app instructions for getting the pinecil ready to flash
## Feel like supporting me?
Well you can buy me a coffee (or rather tea bags or something since i dont drink coffee ;))

<a href="https://www.buymeacoffee.com/spagett" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/default-orange.png" alt="Buy Me A Coffee" height="41" width="174"></a>
## References
- [Blisp](https://github.com/pine64/blisp) - Backend for flashing Pinecil V2
- [Dfu-util](https://dfu-util.sourceforge.net/) - Backend for flashing Pinecil V1
- [Pinecil](https://wiki.pine64.org/wiki/Pinecil) - The Pinecil Wiki page
- [IronOS](https://github.com/Ralim/IronOS) - The firmware running on this soldering iron
