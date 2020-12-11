use std::collections::HashMap;
use std::path::PathBuf;
use sha2::{Sha256, Digest};
use log::{info, warn, debug, error};

trait FileBrowser {
    fn on_file_entry(&mut self, path: PathBuf);

    fn get_hash(&self, path: &PathBuf) -> String {
        let mut file = std::fs::File::open(path).unwrap();
        let mut sha256 = Sha256::new();
        let result = std::io::copy(&mut file, &mut sha256);
        if result.is_ok() {
            let hash = sha256.finalize();
            return format!("{:x}", hash);
        }
        error!("Cannot extract hash {:?}", path);
        panic!("Shutdown");
    }

    fn parse(&mut self, path: &str) {
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
                self.on_file_entry(dir_entry.path());
            }
        }
    }
}

pub struct TargetFileBrowser<'a> {
    files: HashMap<String, String>,
    base_path: &'a str
}

impl<'a> TargetFileBrowser<'a> {
    pub fn new(base_path: &str) -> TargetFileBrowser {
        TargetFileBrowser {
            files: HashMap::new(),
            base_path
        }
    }

    pub fn init(&mut self) {
        self.parse(self.base_path);
    }

    fn add_to_file(&mut self) {}

    fn is_in_files(&self, hash: &String) -> bool {
        return self.files.contains_key(hash);
    }
}

impl FileBrowser for TargetFileBrowser<'_> {
    fn on_file_entry(&mut self, path: PathBuf) {
        let hash = self.get_hash(&path);
        self.files.insert(hash.clone(), path.into_os_string().into_string().unwrap());
        debug!("hash is: {:?}", hash);
    }
}

pub struct SourceFileBrowser<'a> {
    base_path: &'a str,
    target: &'a TargetFileBrowser<'a>
}

impl<'a> SourceFileBrowser<'a> {
    pub fn new(base_path: &'a str, target: &'a TargetFileBrowser) -> SourceFileBrowser<'a> {
        SourceFileBrowser {
            base_path,
            target
        }
    }

    pub fn init(&mut self) {
        self.parse(self.base_path);
    }
}

impl FileBrowser for SourceFileBrowser<'_> {
    fn on_file_entry(&mut self, path: PathBuf) {
        let hash = self.get_hash(&path);
        if !self.target.is_in_files(&hash) {
            debug!("hash is not in files {:?}", hash);
        } else {
            debug!("File with hash {:?} already exists", hash);
            info!("File {:?} already exists in destination.", path);
        }
    }
}