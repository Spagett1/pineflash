use std::process::Command;

use crate::Flasher;

impl Flasher {
    pub fn grab_urls(&mut self) {
        // Grab list of releases
        let command = Command::new("/bin/sh")
            .arg("-c")
            .arg("curl -s https://api.github.com/repos/Ralim/IronOS/releases/latest | \
            grep browser_download_url | \
            cut -d : -f2,3 | \
            sed 's/^ [ \t]*//'")
            .output()
            .expect("Failed to execute command");
        
        let output: String = String::from_utf8(command.stdout).unwrap();

        self.urls.clear();
        for i in output.as_str().lines() {
            self.urls.push(i.to_string());
        }
    }
}