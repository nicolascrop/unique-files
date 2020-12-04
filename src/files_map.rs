use std::collections::HashMap;
use std::path::PathBuf;
use sha2::{Sha256, Digest};
use log::{info, warn, debug};

pub struct FilesMap<'a> {
    files: HashMap<String, String>,
    base_path: &'a String
}

impl<'a> FilesMap<'a> {
    pub fn new(path: &String) -> FilesMap {
        FilesMap {
            files: HashMap::new(),
            base_path: path
        }
    }

    pub fn browse(&self) {
        self.parse(&self.base_path);
    }

    fn parse(&self, path: &String) {
        let entries = std::fs::read_dir(path);
        if entries.is_err() {
            warn!("{:?} is not a valid path", path);
            return;
        }
        for entry in entries.unwrap() {
            if entry.is_err() {
                warn!("Error reading entry {:?}", entry.err());
                continue;
            }
            let dir_entry = entry.unwrap();
            if dir_entry.file_type().is_err() {
                warn!("Could not get file_type for entry {:?}", dir_entry.path());
                continue;
            }
            let file_type = dir_entry.file_type().unwrap();
            info!("file: {:?}", dir_entry.path());
            if file_type.is_dir() {
                self.parse(&dir_entry.path().into_os_string().into_string().unwrap());
            } else {
                let mut file = std::fs::File::open(dir_entry.path()).unwrap();
                let mut sha256 = Sha256::new();
                std::io::copy(&mut file, &mut sha256);
                let hash = sha256.finalize();
                debug!("hash is: {:x}", hash);
            }
        }
    }
}