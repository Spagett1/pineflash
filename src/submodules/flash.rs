use std::fs::File;
use std::time::Duration;
use std::{process::Command};
use std::io::{Read, Cursor};
use std::path::{Path, PathBuf};
#[cfg(target_family = "unix")]
static DFU_COMMAND: &str = "dfu-util";
#[cfg(target_family = "unix")]
static BLISP_COMMAND: &str = "blisp";
#[cfg(target_os = "windows")]
static DFU_COMMAND: &str = "dfu-util.exe";
#[cfg(target_os = "windows")]
static BLISP_COMMAND: &str = "blisp.exe";

use crate::Flasher;

impl Flasher {
    pub fn flash(&mut self) {
        let mut firmware_path = "".to_string();
            if self.config.version != "Custom".to_string() {


                #[cfg(target_family = "windows")]
                let path: PathBuf = ["c:\\", "Users", whoami::username().as_str(), "AppData", "Local", "Temp", format!("{}-{}.zip", self.config.version, self.config.int_name).as_str()].iter().collect();
                #[cfg(target_family = "unix")]
                let path: PathBuf = ["/tmp", format!("{}-{}.zip", self.config.version, self.config.int_name).as_str()].iter().collect();


                #[cfg(target_family = "windows")]
                let target: PathBuf = ["c:\\", "Users", whoami::username().as_str(), "AppData", "Local", "Temp", format!("{}-{}", self.config.version, self.config.int_name).as_str()].iter().collect();
                #[cfg(target_family = "unix")]
                let target: PathBuf = ["/tmp", format!("{}-{}", self.config.version, self.config.int_name).as_str()].iter().collect();

                let mut file = File::open(path).unwrap();
                let mut data = Vec::new();
                file.read_to_end(&mut data).unwrap();

                zip_extract::extract(Cursor::new(data), &target, true).unwrap();
                self.config.logs.push_str("File extracted successfully\n");


                if cfg!(unix) && self.config.int_name == "Pinecil" {
                    firmware_path = format!("{}/{}_{}.dfu", target.as_os_str().to_str().unwrap(), self.config.int_name, self.config.lang);
                } else if cfg!(unix) && self.config.int_name == "Pinecilv2" {
                    firmware_path = format!("{}/{}_{}.bin", target.as_os_str().to_str().unwrap(), self.config.int_name, self.config.lang);
                } else {
                    // Do windows functionality here.
                    firmware_path = format!("{}\\{}_{}.dfu", target.as_os_str().to_str().unwrap(), self.config.int_name, self.config.lang);
                }

            } else if self.config.picked_path != None {
                firmware_path = self.config.picked_path.as_ref().unwrap().to_string();
            }  else {
                self.toasts.error("Please select a file or firmware version");
            }

            self.config.logs.push_str(format!("Attempting to flash {} with the firmware {}\n", self.config.int_name, firmware_path).as_str());
            if self.config.int_name == "Pinecil" {
                let command = Command::new(DFU_COMMAND)
                    .arg("-D")
                    .arg(firmware_path)
                    .output()
                    .expect("Could not flash soldering iron");
                let output: String = String::from_utf8(command.stdout).unwrap();
                let output_err: String = String::from_utf8(command.stderr).unwrap();
                self.toasts.dismiss_all_toasts();
                if command.status.success() {
                    self.toasts.info("Flashing completed").set_duration(Some(Duration::from_secs(5))).set_closable(false);
                } else {
                    self.toasts.error("Flashing failed, is your pinecil plugged in?").set_duration(Some(Duration::from_secs(5))).set_closable(false);
                }
                self.config.logs.push_str(format!("{}{}\n", output, output_err).as_str());
            } else if self.config.int_name == "Pinecilv2" {
                let command = Command::new(BLISP_COMMAND)
                    .arg("write")
                    .arg("-c")
                    .arg("bl70x")
                    .arg("--reset")
                    .arg(firmware_path)
                    .output()
                    .expect("Could not flash soldering iron");

                let output: String = String::from_utf8(command.stdout).unwrap();
                let output_err: String = String::from_utf8(command.stderr).unwrap();
                self.toasts.dismiss_all_toasts();
                if !output_err.as_str().contains("Device not found") && !output_err.as_str().contains("Failed") {
                    self.toasts.info("Flashing completed").set_duration(Some(Duration::from_secs(5))).set_closable(false);
                } else {
                    self.toasts.error("Flashing failed, is your pinecil plugged in?").set_duration(Some(Duration::from_secs(5))).set_closable(false);
                }
                self.config.logs.push_str(format!("{}{}\n", output, output_err).as_str());
                
                // Very ugly way of reseting the program
                self.config.version = "Select".to_string();
                self.config.fancy_names = vec![];
                self.config.code_names = vec![];
                self.config.versions_checked = false;
                self.config.vers = vec![];
                self.config.promise = None;
                self.config.promise_2 = None;
                self.config.promise_3 = None;
                self.config.download_metadata = false;
                self.config.download = false;
                self.config.download_notify = true;
                self.config.download_firm_notify = true;
                self.config.picked_path = None;
                self.config.ready_to_flash = false;


            }
            
    }
}