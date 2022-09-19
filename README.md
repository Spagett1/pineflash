# IronOs-Flasher
A tool to flash ironos to the pinecil and ts irons 

# Dependancies
```
dfu-util
curl (you probably have this installed already)
```

# Current State
This will allow you to choose an iron and ironOs version and download it (current path is /tmp)

It should flash to the pinecil however mine hasnt actually arrived yet so there is a possibility i have gotten it wrong there. 

I also want to experiment with the async library in rust so the whole program doesnt freeze when downloading.

Also some sort of toast while stuff is downloading, flashing, etc.
