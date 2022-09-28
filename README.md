# PineFlash
A tool to flash ironos to the pinecil and in the future other pine products
![image](https://user-images.githubusercontent.com/77225642/192753666-1a0e2bf4-b5ec-4e35-ba31-aae9043e04b9.png)

# Dependancies
```
dfu-util
curl (you probably have this installed already)
```
# Installing
Go over to https://github.com/Laar3/PineFlash/releases and download the latest version for your os

Make sure `dfu-util` is installed

Move the binary into your path 

`mv ./pineflash_linux_x86 /bin/pineflash`

Then to run 

`pineflash`

# Building
Install all the dependancies in addition to rust 

After that run 
```
cargo build --release
```
The resulting binary will be in `target/release/pineflash`, this can be moved into your path or just run as a portable executable

# Current State
This will allow you to choose an iron and ironOs version and download it (current path is /tmp)

It should flash to the pinecil however mine hasnt actually arrived yet so there is a possibility i have gotten it wrong there. 

# Todo
Some sort of toast while stuff is downloading, flashing, etc.
