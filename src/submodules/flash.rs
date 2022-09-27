use std::fs::File;
use std::time::Duration;
use std::{process::Command};
use std::io::{Read, Cursor};
use std::path::Path;

use crate::Flasher;
// use unzip_rs::{self, unzip};

impl Flasher {
    pub fn flash(&mut self) {
        // self.config.firmware_download = false;
        let mut firmware_path = "".to_string();
        if self.config.version != "Custom".to_string() {
            let path = format!("/tmp/{}-{}.zip", self.config.version, self.config.int_name);

            let target= format!("/tmp/{}-{}/", self.config.version, self.config.int_name);
            let mut file = File::open(path).unwrap();
            let mut data = Vec::new();
            file.read_to_end(&mut data).unwrap();
            
            let target_dir = Path::new(target.as_str()); // Doesn't need to exist

            zip_extract::extract(Cursor::new(data), target_dir, true).unwrap();
            firmware_path = format!("{}/{}_{}.dfu", target, self.config.int_name, self.config.lang);

        } else if self.config.picked_path != None {
            firmware_path = self.config.picked_path.as_ref().unwrap().to_string();
        }  else {
            self.toasts.error("Please select a file or prepicked version");
        }

        if self.config.iron == "Pinecil V1" {
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
        self.toasts.dismiss_all_toasts();
        self.toasts.info("Flashing completed").set_duration(Some(Duration::from_secs(5))).set_closable(false);
            
    }
}