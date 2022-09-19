use std::fs::File;
use std::{process::Command};
use std::io::{Read, Cursor};
use std::path::Path;

use crate::Flasher;
// use unzip_rs::{self, unzip};

impl Flasher {
    pub fn flash(&mut self) {
        let path = format!("/tmp/{}-{}.zip", self.version, self.iron);

        let target= format!("/tmp/{}-{}/", self.version, self.iron);
        let mut file = File::open(path).unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        
        let target_dir = Path::new(target.as_str()); // Doesn't need to exist

        zip_extract::extract(Cursor::new(data), &target_dir, true).unwrap();

        let firmware_path = format!("{}/{}_{}.dfu", target, self.iron, self.lang);
        let flash_command = format!("dfu-util -D \"{}\"", firmware_path);
        let command = Command::new("/bin/sh")
            .arg("-c")
            .arg(flash_command)
            .output()
            .expect("Could not flash soldering iron");
        println!("{:?}", command);
        let output: String = String::from_utf8(command.stdout).unwrap();
        let output_err: String = String::from_utf8(command.stderr).unwrap();
        println!("{}", output);
        println!("{}", output_err)
            
    }
}