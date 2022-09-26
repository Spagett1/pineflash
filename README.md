# PineFlash
A tool to flash ironos to the pinecil and in the future other pine products
![image](https://user-images.githubusercontent.com/77225642/192240698-96690330-82b2-42fe-beed-0ceb097d49b8.png)

# Dependancies
```
dfu-util
curl (you probably have this installed already)
```

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
