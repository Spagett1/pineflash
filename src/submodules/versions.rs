use std::process::Command;

use crate::Flasher;

impl Flasher {
    pub fn grab_vers(&mut self) {
        let command = Command::new("/bin/sh")
            .arg("-c")
            .arg("curl -s https://api.github.com/repos/Ralim/IronOS/releases | grep tag_name | head -n 5 | cut -d\\\" -f4")
            .output()
            .expect("Failed to get amount of releases");
        
        let output: String = String::from_utf8(command.stdout).unwrap();

        self.vers.clear();
        for i in output.as_str().lines() {
            self.vers.push(i.to_string());
        }
    }
}