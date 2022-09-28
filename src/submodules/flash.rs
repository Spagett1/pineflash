use std::fs::File;
use std::time::Duration;
use std::{process::Command};
use std::io::{Read, Cursor};
use std::path::{Path, PathBuf};
#[cfg(target_os = "macos")]
static DFU_COMMAND: &str = "dfu-util";
#[cfg(target_os = "linux")]
static DFU_COMMAND: &str = "dfu-util";
#[cfg(target_os = "windows")]
static DFU_COMMAND: &str = "dfu-util.exe";

use crate::Flasher;
// use unzip_rs::{self, unzip};

impl Flasher {
    pub fn flash(&mut self) {
        // self.config.firmware_download = false;
        let mut firmware_path = "".to_string();
        if self.config.version != "Custom".to_string() {

            let mut path;
            if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
                // path = format!("/tmp/{}-{}.zip", self.config.version, self.config.int_name);
                path = PathBuf::from("/tmp");
                path.push(format!("{}-{}.zip", self.config.version, self.config.int_name));
            }  else {
                path = PathBuf::from(std::env::var("TEMP").unwrap());
                path.push(format!("{}-{}.zip", self.config.version, self.config.int_name));
                //  path = PathBuf::from("c:\\");
                //  path.push("users");
                //  path.push(whoami::username());
                //  path.push("Temp");               // path = format!("c:\\users\\{}\\Temp\\{}-{}.zip", whoami::username(), self.config.version, self.config.int_name);
            }

            let mut target;
            if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
                target = PathBuf::from("/tmp");
                target.push(format!("{}-{}", self.config.version, self.config.int_name));
            } else {
                target = PathBuf::from(std::env::var("TEMP").unwrap());
                target.push(format!("{}-{}", self.config.version, self.config.int_name));
            }

            
            let target_dir = Path::new(&target); // Doesn't need to exist

            let mut file = File::open(path).unwrap();
            let mut data = Vec::new();
            file.read_to_end(&mut data).unwrap();

            zip_extract::extract(Cursor::new(data), target_dir, true).unwrap();
            self.config.logs.push_str("File extracted successfully\n");


            if cfg!(unix) {
                firmware_path = format!("{}/{}_{}.dfu", target.as_os_str().to_str().unwrap(), self.config.int_name, self.config.lang);
            } else {
                firmware_path = format!("{}\\{}_{}.dfu", target.as_os_str().to_str().unwrap(), self.config.int_name, self.config.lang);
            }

        } else if self.config.picked_path != None {
            firmware_path = self.config.picked_path.as_ref().unwrap().to_string();
        }  else {
            self.toasts.error("Please select a file or prepicked version");
        }

        if self.config.iron == "Pinecil V1" {
            let command = Command::new(DFU_COMMAND)
                .arg("-D")
                .arg(firmware_path)
                .output()
                .expect("Could not flash soldering iron");
            let output: String = String::from_utf8(command.stdout).unwrap();
            let output_err: String = String::from_utf8(command.stderr).unwrap();
            self.config.logs.push_str(format!("{}{}\n", output, output_err).as_str());
        }
        self.toasts.dismiss_all_toasts();
        self.toasts.info("Flashing completed").set_duration(Some(Duration::from_secs(5))).set_closable(false);
            
    }
}