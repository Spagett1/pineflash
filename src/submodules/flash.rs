use std::fs::File;
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

        // The third parameter allows you to strip away toplevel directories.
        // If `archive` contained a single folder, that folder's contents would be extracted instead.
        zip_extract::extract(Cursor::new(data), &target_dir, true).unwrap();
            

        // unzip(, target_path)
        // let command = Command::new("/bin/sh")
            // .arg("-c")
            // .arg(com)
            // .output()
            // .expect("Failed to get amount of releases");
    }
}