
<a href="https://github.com/Spagett1/PineFlash#pineflash"><img src="https://hits.seeyoufarm.com/api/count/incr/badge.svg?url=https%3A%2F%2Fgithub.com%2FSpagett1%2FPineFlash&count_bg=%23187BC0&title_bg=%23555555&icon=&icon_color=%23E7E7E7&title=hits&edge_flat=true"/></a>
[![GitHub all downloads](https://img.shields.io/github/downloads/spagett1/pineflash/total?color=187BC0&style=flat-square)](https://github.com/Spagett1/PineFlash/releases/)
[![GitHub release (latest by date)](https://img.shields.io/github/v/release/spagett1/pineflash?color=187BC0&style=flat-square)](https://github.com/Spagett1/PineFlash/releases/)


# PineFlash

<img src="https://user-images.githubusercontent.com/77225642/234575696-e6ea62ae-7189-4a9b-bcfb-7f0c36271ace.png" align="right" width="425" style="float:left">

A GUI tool to flash IronOS to the Pinecil V1, V2 and future other pine products.  

## Features
* Auto-fetch the newest stable release of IronOS firmware.
* Allows manual installs of beta versions using a browse to file feature.
* Selectable options: pick the iron type, pick the firmware version and download it.
* Boot logo art upload supported for V1 (use custom & folder icon)

<br clear="both" />

## Supported Devices 
 | System  |<img width="17" src="https://cdn.simpleicons.org/Linux/187BC0" /> Linux  | <img width="15" src="https://cdn.simpleicons.org/Apple/187BC0" /> MacOS|  <img width="15" src="https://cdn.simpleicons.org/Windows11/187BC0" /> Windows|
 | :-----: | :-----: | :-----: | :-----: |
 | Pinecil V1 |<img width="18" src="https://cdn.simpleicons.org/cachet/187BC0" />|<img width="18" src="https://cdn.simpleicons.org/cachet/187BC0" />| <img width="18" src="https://cdn.simpleicons.org/cachet/187BC0" />  |
 | Pinecil V2 | <img width="18" src="https://cdn.simpleicons.org/cachet/187BC0" />   | <img width="18" src="https://cdn.simpleicons.org/cachet/187BC0" />  |  <img width="18" src="https://cdn.simpleicons.org/cachet/187BC0" />  |
<br clear="both" />


# :desktop_computer: Install Options


1. Easy install: use premade binaries, currently only available for Linux x86 distros and Windows x86.

2. Build from Code: recommended if you are on MacOS, an ARM device, or doing development.

<details>
  <summary>
   
## :clamp: Premade Binaries 
 </summary>

## <img width="18" src="https://cdn.simpleicons.org/Windows11/187BC0" /> Windows
Download the latest pineflash exe file from the [releases page](https://github.com/Spagett1/PineFlash/releases).

Then just double click it.

### <img width="18" src="https://cdn.simpleicons.org/RedHat/" /> RedHat distros (Fedora, Centos, Nobara, Rocky, etc.)
Download the latest pineflash .rpm file from the [releases page](https://github.com/Spagett1/PineFlash/releases).

 Then just run.
 ```
 sudo dnf install ./pineflash-*.x86_64.rpm
 ```
 
### <img width="18" src="https://cdn.simpleicons.org/ArchLinux/187BC0" /> Arch based distros (Arch, Artix, Manjaro, Endeavor, etc.)

Download the latest pineflash pkg.tar.zst file from the [releases page](https://github.com/Spagett1/PineFlash/releases).

Then simply run.
```
sudo pacman -U pineflash-*-x86_64.pkg.tar.zst
```

### <img width="18" src="https://cdn.simpleicons.org/Debian" /> Debian based distros (Debian, Ubuntu, PopOs, etc.)
Download the latest pineflash .deb file from the [releases page](https://github.com/Spagett1/PineFlash/releases).

Then just run.
```
sudo apt install ./pineflash_*_amd64.deb
```

### <img width="18" src="https://cdn.simpleicons.org/Linux/CC5500" /> Other Linux x86 distro's
Download the latest pineflash .tar.xz file from the [releases page](https://github.com/Spagett1/PineFlash/releases).

Extract the file.
```
tar -xf ./pineflash-*-x86_64.tar.xz
```

And copy the contents into your system 
```
doas cp -r ./usr /
```
> **_NOTE:_**  Make sure you install dfu-util manually if you have a pinecil v1 and choose this option. Window manager users should ensure they have a pokit agent installed and enabled.

### <img width="18" src="https://cdn.simpleicons.org/Apple/818589" /> MacOs
Sorry we dont have built apps for you yet, head to the build from source section and use the unix install script. 

> **_NOTE:_** You will also need to run pineflash from the terminal, this is on the todo list to get fixed.

</details>
<div style="clear:both;">&nbsp;</div>

 
<details>
  <summary>
   
## :building_construction: Build from code 
 </summary>


Use this build method if the premade binaries do not support your architecture or you have dev purposes.

### :bookmark_tabs: Build Dependancies

Install these if you don't have them (not needed if using the PKGBUILD).


<details>
  <summary>
<img width="17" src="https://cdn.simpleicons.org/windowsterminal/F46D01" /> General dependancy list
</summary>

This list covers linux distros which are not named below and macos.

If your operating system has its own section, then please go there; it has package names tailored to your distro.

If you had to install more dependencies to get it to work, please open an issue to let us know the specific OS and packages you used.

```
cmake
rust 
git
dfu-util - for pinecil V1 support
polkit - Linux only
gcc         
```
</details>
<div style="clear:both;">&nbsp;</div>

<details>
  <summary>
<img width="17" src="https://cdn.simpleicons.org/debian/A81D33" /> Dependencies for Debian
</summary>

```
cmake
rust-all (alternatively go to https://rustup.rs/)
git
dfu-util - for pinecil V1 support
policykit-1
g++
pkg-config 
libglib2.0-dev
build-essential    
libfontconfig-dev 
fontconfig-config  
libgdk3.0-cli-dev
libatk1.0-0   
libatk1.0-dev       
libgtk-3-dev             
```
This line will install everything:
```
sudo apt install cmake rust-all git dfu-util policykit-1 g++ pkg-config libglib2.0-dev build-essential libfontconfig-dev fontconfig-config libgdk3.0-cli-dev libatk1.0-0 libatk1.0-dev libgtk-3-dev             
```

</details>
<div style="clear:both;">&nbsp;</div>
<details>
  <summary>
<img width="17" src="https://cdn.simpleicons.org/archlinux/187BC0" /> Dependencies for Arch
 </summary>

#### Runtime dependancies
```
dfu-util
fontconfig
glibc
gtk3
polkit
```
#### Build dependancies
```
base-devel
cargo-ndk # To verify some integrity checksums of rust modules
git
optipng
pkgconf
rust
```
This line will install everything:
```
sudo pacman -S --needed cmake rust git dfu-util polkit gcc pkgconf glibc base-devel fontconfig gtk3
```

</details>
<div style="clear:both;">&nbsp;</div>
<details>
  <summary>
  
### :toolbox: Build option 1, handy scripts

 </summary>
 
Handy scripts will compile and install PineFlash for you.

### <img width="17" src="https://cdn.simpleicons.org/Linux/187BC0" /> Build Unix from script. (Macos and Linux)
1. To build from source code, first install build dependencies.
2. Download the source code with the following commands.
```
git clone https://github.com/Spagett1/PineFlash/
cd PineFlash
```
3. Run the `generic_unix_install.sh` file which will build and install Pineflash.

### <img width="17" src="https://cdn.simpleicons.org/archlinux/187BC0" />  Build on Arch based distro's (Arch, Artix, Manjaro, Endeavor, Arch Arm, etc.) 
1. All dependancies will be handled by the PKGBUILD
2. PineFlash is in the aur so you can install it just like any other aur package.
```
git clone https://aur.archlinux.org/pineflash-git.git
cd pineflash-git
makepkg -si
```
3. Alternatively just use your favourite aur helper.
</details>
<div style="clear:both;">&nbsp;</div>

<details>
  <summary>
   
### :weight_lifting_man: Build option 2: manual build
 </summary>

Old school style, this is recommended if you have issues with the scripts or want to help develop PineFlash.
 
1. Install all the build dependancies listed above.

2. Download the source code.

```
git clone https://github.com/Spagett1/PineFlash/
cd PineFlash 
```

4. Download blisp which is needed for pinecil V2 support, alternatively compile it if you are not on a 64 bit computer. [Instructions]("https://github.com/pine64/blisp")

In the following instructions replace `platform` with your operating system (`linux` or `macos`)
```
curl -L "https://github.com/pine64/blisp/releases/download/v0.0.3/blisp-platform64-v0.0.3.zip" -o "blisp-platform64-v0.0.3.zip"
unzip "blisp-platform-v0.0.3.zip"
sudo mv ./blisp /usr/local/bin/blisp
```
:dart: Important: Don't forget to add blisp to your path 

5. Then build pineflash itself
```
cargo build --release
```
6. The resulting binary will be in `target/release/pineflash`, this can be moved into your path (`/usr/bin/pineflash`) or just run as a portable executable.

7. Then copy the `assets/Pineflash.desktop` file to `/usr/share/applications` and copy `assets/pine64logo.png` to `/usr/share/pixmaps` for the shortcut to show up in launchers. (This does not apply to MacOs, you will have to run pineflash from the terminal for now, sorry.)

8. Just run the program by typing it into the terminal.
```
pineflash
```
Alternatively just run it from your app launcher (unless your on macos).

</details>
<div style="clear:both;">&nbsp;</div>
 
 
</details>
<div style="clear:both;">&nbsp;</div>

## :runner: Run 

<img width="17" src="https://cdn.simpleicons.org/Linux/CC5500" /> Linux: Pineflash should appear in app launcher options. Alternatively, you can run the command:  

     `pineflash`

<img width="17" src="https://cdn.simpleicons.org/Apple/818589" /> MacOS: Sorry, no launcher icon yet, youll need to run this command to run pineflash.

     `pineflash`

<img width="17" src="https://cdn.simpleicons.org/Windows/187BC0" /> Windows: Just run pineflash from the start menu.


## Boot Logo Art

- Select Custom from version drop-down, then click on the folder icon to browse to the art file you want to load.
- Currently only V1 supports Boot logos.
- See [IronOS-Meta](https://github.com/Ralim/IronOS-Meta/) to get premade art or details on how to make custom art.
- [Work in progress](https://github.com/Ralim/IronOS/issues/1373#issuecomment-1414925011) by Ralim to get blisp to support logos for V2.
- Presently, the only method for V2 is to make a custom version of IronOS firmware with logo art embeded in the code; this is for advanced users comfortable Github and code changes [see here](https://github.com/Ralim/IronOS-Meta/issues/32#issuecomment-1505172982).

<br>

<details>
  <summary>
 
 ## :electric_plug: Connect Pinecil to a PC
 </summary>

1. To do the firmware update, connect one end of a USB cable to the PC.
2. Then, hold down the `[-]` button **before** plugging the usb-c cable to the back of Pinecil.
3. Keep holding the `[-]` for ~10 seconds more before releasing the button. If you correctly entered flashing mode, the screen will be black/empty. If not, do it again, flip the cable, or try another cable, different port, or a different PC.
4. See [Pinecil Wiki](https://wiki.pine64.org/wiki/Pinecil_Firmware) firmware details if you get stuck.
</details>
<div style="clear:both;">&nbsp;</div>

## :spiral_calendar: Todo

- [x] Windows support.
- [ ] MacOS premade binary
- [ ] Launcher icon for macos, easier method to install
- [x] Improve UI (colors, design, workflow).
- [ ] Improve Light Mode.
- [x] In app instructions for connecting pinecil to pc.
- [ ] V2 boot logo art support when blisp supports it.

## :tea: Feel like supporting me?

Well you can buy me a coffee, or rather tea bags since i dont drink coffee ;)

<a href="https://www.buymeacoffee.com/spagett" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/default-orange.png" alt="Buy Me A Coffee" height="41" width="174"></a>

## :book: References

- [Blisp](https://github.com/pine64/blisp) - Backend for flashing Pinecil V2
- [Dfu-util](https://dfu-util.sourceforge.net/) - Backend for flashing Pinecil V1
- [Pinecil](https://wiki.pine64.org/wiki/Pinecil) - The Pinecil Wiki page
- [IronOS](https://github.com/Ralim/IronOS) - The firmware running on this soldering iron
- [PineSAM](https://github.com/builder555/PineSAM) - A cool Bluetooth app to control Pinecil V2 from any browser
- [Egui](https://github.com/emilk/egui) - The awesome GUI toolkit used to make this program
- [InnoSetup](https://github.com/jrsoftware/issrc) - The software i use to generate windows releases
- [FPM](https://github.com/jordansissel/fpm) - The tool i use to generate linux releases for all sorts of distros
 
 ## :dash: Stay fluxy my friends.
 <img src="https://user-images.githubusercontent.com/77225642/229288128-e6993505-47a2-4437-92cf-7b2a5de10677.png" width="425">
 
[Source](https://www.reddit.com/r/PINE64official/comments/xk9vxu/most_interesting_man_in_the_world_i_dont_always/)
