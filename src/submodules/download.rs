use std::{process::Command};
use crate::Flasher;

impl Flasher {
    pub fn download (&mut self) {
        let com = format!("curl -sL https://github.com/Ralim/IronOS/releases/download/{}/{}.zip -o /tmp/{}-{}.zip", self.config.version, self.config.iron, self.config.version, self.config.iron);
        let command = Command::new("/bin/sh")
            .arg("-c")
            .arg(com)
            .output()
            .expect("Failed to get amount of releases");
        
        let output: String = String::from_utf8(command.stdout).unwrap();
        println!("{}", output);

    }
}