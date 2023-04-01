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
<br clear="both" />

## :bookmark_tabs: Dependancies
```
polkit
dfu-util - Pinecil V1 only
```
### Disclaimer: does not currently work on wayland.


## :desktop_computer: Install Options

1. Pre-made Binaries: Currently only available for x86 Linux distros.

2. Build from Code: Recommended for development or if you are on an ARM device or MacOs.


<details>
  <summary>
   
# :computer: Premade Binaries 
 </summary>
 
## <img width="17" src="https://cdn.simpleicons.org/Linux/000000" /> Arch based distros (Arch, Artix, Manjaro, Endeavor)

Head over to [releases](https://github.com/Spagett1/PineFlash/releases).

Download the latest .tar.zst file.

Then simply run.
```
sudo pacman -U ./pineflash-*-x86.tar.zst 
```

## <img width="17" src="https://cdn.simpleicons.org/Linux/000000" /> Other x86 Linux distro's. 

Make sure you have the needed [dependancies](https://github.com/Spagett1/PineFlash/releases) installed

Download the latest pineflash_linux_x86_<version>.zip file from the [releases page](https://github.com/Spagett1/PineFlash/releases).

Then extract and install it.
```
unzip ./pineflash_linux_x86_*.zip
sudo cp -r usr/* /usr/
```

</details>
<div style="clear:both;">&nbsp;</div>

 
<details>
  <summary>
   
# :building_construction: Build from code 
 </summary>


This is the same PineFlash as the pre-made binaries [here](https://github.com/Spagett1/PineFlash/releases/). Install this if the binaries do not support your architecture or you have dev purposes.

## :bookmark_tabs: Build Dependancies

Install all of this if you don't have it (not needed if using the PKGBUILD).
```
git
rust
cmake
polkit
gtk3 (arch based distros) / libgtk-3-dev (debian based distros)
dfu-util # For pinecil v1 support. 
```
<details>
  <summary>
   
## :toolbox: Build Option 1, handy scripts
 </summary>
 
Use the handy scripts will compile and install PineFlash for you.

## <img width="17" src="https://cdn.simpleicons.org/Linux/000000" /> Build Linux from script.
1. To build from source code, first install build dependencies.
2. Extract the source code tar.gz from the newest Assets in [releases here](https://github.com/Spagett1/PineFlash/releases/)
3. Run the `generic_linux_install.sh` file which will build and install Pineflash.

## <img width="17" src="https://cdn.simpleicons.org/archlinux/000000" />  Build on Arch based distro's
1. All dependancies will be handled by the PKGBUILD
2. You can use the PKGBUILD which will handle everything for you.
3. Just run `makepkg -si` in the main directory to build and install it.
</details>
<div style="clear:both;">&nbsp;</div>


<details>
  <summary>
   
## :man_factory_worker: Build Option 2, manual build
 </summary>

Old school style, this is recommended if you have issues with the scripts or want to help develop PineFlash.
 
1. Install all the dependencies.

2. download the git submodules 
```
git submodule update --init --recursive
```
3. build blisp which is needed for pinecil V2 support 
```
cd blisp
mkdir build
cd build
cmake -DBLISP_BUILD_CLI=ON ..
cmake --build .
sudo mv ./tools/blisp/blisp /usr/bin/ #Or some other global path.
```
:dart: Important: Don't forget to add blisp to your path

4. Then build pineflash itself
```
cargo build --release
```
5. The resulting binary will be in `target/release/pineflash`, this can be moved into your path (`/usr/bin/pineflash`) or just run as a portable executable.

6. Then copy the Pineflash.desktop file to `/usr/share/applications` and copy `assets/pine64logo.png` to `/usr/share/pixmaps` for the shortcut to show up in launchers.

7. On linux, root permissions are needed for dfu-util and blisp if running from the terminal. In order to solve this you need to run the program with the following command  
`pkexec env DISPLAY=$DISPLAY XAUTHORITY=$XAUTHORITY pineflash`.   
If you use the Gui app, then don't worry about it. It's already in the .desktop file and not necessary.

</details>
<div style="clear:both;">&nbsp;</div>
 
 
</details>
<div style="clear:both;">&nbsp;</div>

## :runner: Run 

Linux: Pineflash should just appear in your app launcher. Alternatively you can run `pkexec env DISPLAY=$DISPLAY XAUTHORITY=$XAUTHORITY pineflash` from the command line.

MacOS: Simply running `pineflash` will work fine as it doesn't need root privledges. Sorry, no launcher icon yet. 
<br><br>

 
 ## :electric_plug: Connect Pinecil to a PC

1. To do the firmware update, connect one end of a USB cable to the PC.
2. Then, hold down the `[-]` button **before** plugging the usb-c cable to the back of Pinecil.
3. Keep holding the `[-]` for ~10 seconds more before releasing the button. If you correctly entered flashing mode, the screen will be black/empty. If not, do it again, flip the cable, or try another cable, different port, or a different PC.
4. See [Pinecil Wiki](https://wiki.pine64.org/wiki/Pinecil_Firmware) firmware details if you get stuck.
 
## :spiral_calendar: Todo

- [ ] Windows support.
- [ ] In app instructions for getting the pinecil ready to flash.
- [ ] Changing boot logo support.
- [ ] Improve UI (colors, design, workflow).
- [ ] Launcher icon for macos and an easier way to install it there.

## :tea: Feel like supporting me?

Well you can buy me a coffee, or rather tea bags or something since i dont drink coffee ;)

<a href="https://www.buymeacoffee.com/spagett" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/default-orange.png" alt="Buy Me A Coffee" height="41" width="174"></a>

## :book: References

- [Blisp](https://github.com/pine64/blisp) - Backend for flashing Pinecil V2
- [Dfu-util](https://dfu-util.sourceforge.net/) - Backend for flashing Pinecil V1
- [Pinecil](https://wiki.pine64.org/wiki/Pinecil) - The Pinecil Wiki page
- [IronOS](https://github.com/Ralim/IronOS) - The firmware running on this soldering iron
- [PineSAM](https://github.com/builder555/PineSAM) - a cool Bluetooth app to control Pinecil V2 from any browser
