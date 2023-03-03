![HIT Count](https://img.shields.io/endpoint?color=blue&url=https%3A%2F%2Fhits.dwyl.com%2Fspagett1%2Fpineflash.json)
![GitHub all releases](https://img.shields.io/github/downloads/spagett1/pineflash/total?color=blue)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/spagett1/pineflash)


# PineFlash
![image](https://user-images.githubusercontent.com/77225642/192753666-1a0e2bf4-b5ec-4e35-ba31-aae9043e04b9.png)

A tool to flash ironos to the Pinecil V1 and in the future other pine products.

## Platforms
- [x] Linux
- [x] MacOS

## Devices
- [x] Pinecil V1

## Dependancies
```
dfu-util
polkit
```

### Disclaimer: This does not currently work on wayland.
## Installing
Go over to https://github.com/Laar3/PineFlash/releases, intructions can be found there.

Then to run 
`pkexec env DISPLAY=$DISPLAY XAUTHORITY=$XAUTHORITY pineflash`
On macos simply running `pineflash` will work fine as it doesnt need root privledges. 

## Building
## Build Dependancies
```
git
rust
gtk3 (arch based distros) / libgtk-3-dev (debian based distros)
```

## Linux
On linux just run the `generic_linux_install.sh` file which will build and install Pineflash.

### Arch based distro's
All dependancies will be handled by the PKGBUILD
You can use the PKGBUILD which will handle everything for you.
Just run `makepkg -si` in the main directory to build and install it.


## Manual build
```
cargo build --release
```
The resulting binary will be in `target/release/pineflash`, this can be moved into your path (`/usr/bin/pineflash`) or just run as a portable executable.

Then copy the Pineflash.desktop file to `/usr/share/applications` and copy `assets/pine64logo.png` to `/usr/share/pixmaps` for the shortcut to show up in launchers.

Note, on linux root permissions are needed for dfu-util, in order to solve this you need to run the program with the following command if running from the terminal `pkexec env DISPLAY=$DISPLAY XAUTHORITY=$XAUTHORITY pineflash`.

This is already in the .desktop file so dont worry about that if you just use it.


## Current State
* This will allow you to choose an iron and IronOs firmware version and download it (current path is /tmp) without needing to grab it separately from Ralim's IronOS.
* This currently only supports unix-like operating systems (macos and linux), windows support is in the works.
* To connect Pinecil V1 to the computer to do the firmware update, hold down the `[-]` button before plugging the usb-c cable to the back of Pinecil. Keep holding the `[-]` for ~10 seconds more before releasing the button. If you correctly entered flashing mode, the screen will be black/empty.

 

## Todo

~~Some sort of toast while stuff is downloading, flashing, etc.~~

- [x] Windows support
- [x] V2 support 
- [x] In app instructions for getting the pinecil ready to flash

## References

- [Pinecil](https://wiki.pine64.org/wiki/Pinecil) - The Pinecil Wiki page
- [IronOS](https://github.com/Ralim/IronOS) - The firmware running on this soldering iron
