## Disclaimer: This does not currently work on wayland.

# PineFlash
A tool to flash ironos to the pinecil and in the future other pine products
![image](https://user-images.githubusercontent.com/77225642/192753666-1a0e2bf4-b5ec-4e35-ba31-aae9043e04b9.png)

# Dependancies
```
dfu-util
polkit
```

# Installing
Go over to https://github.com/Laar3/PineFlash/releases, intructions can be found there.

Then to run 
`pkexec env DISPLAY=$DISPLAY XAUTHORITY=$XAUTHORITY pineflash`
On macos simply running `pineflash` will work fine as it doesnt need root privledges. 

# Building
Install all the dependancies in addition to rust 

# Build Dependancies
```
git
rust
gtk3 (arch based distros) / libgtk-3-dev (debian based distros)
```

# Linux
On linux just run the `generic_linux_install.sh` file which will build and install Pineflash.

## Arch based distro's
You can use the PKGBUILD which will handle everything for you.
Just run `makepkg -si` in the main directory to build and install it.


# Manual build
```
cargo build --release
```
The resulting binary will be in `target/release/pineflash`, this can be moved into your path (`/usr/bin/pineflash`) or just run as a portable executable.

Then copy the Pineflash.desktop file to `/usr/share/applications` and copy `assets/pine64logo.png` to `/usr/share/pixmaps` for the shortcut to show up in launchers.

Note, on linux root permissions are needed for dfu-util, in order to solve this you need to run the program with the following command if running from the terminal `pkexec env DISPLAY=$DISPLAY XAUTHORITY=$XAUTHORITY pineflash`.

This is already in the .desktop file so dont worry about that if you just use it.


# Current State
This will allow you to choose an iron and ironOs version and download it (current path is /tmp

This currently only supports unixlike operating systems (macos and linux), windows support is in the works.

It should flash to the pinecil however mine hasnt actually arrived yet so there is a possibility i have gotten it wrong there. 

# Todo

~~Some sort of toast while stuff is downloading, flashing, etc.~~

Windows support

V2 support 

In app instructions for getting the pinecil ready to flash
