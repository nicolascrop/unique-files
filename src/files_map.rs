use std::collections::HashMap;
use std::path::PathBuf;
use sha2::{Sha256, Digest};
use log::{info, warn, debug, error};

trait FileBrowser {
    fn on_file_entry(&mut self, path: &PathBuf);

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

    fn parse(&mut self, path: &String) {
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
                self.on_file_entry(&dir_entry.path());
            }
        }
    }
}

struct TargetFileBrowser<'a> {
    files: HashMap<String, String>,
    base_path: String,
    preserve_path: bool,
    on_progress: &'a dyn Fn(&PathBuf)
}

impl<'a> TargetFileBrowser<'a> {
    pub fn new(base_path: &str, preserve_path: bool, on_progress: &'a dyn Fn(&PathBuf)) -> TargetFileBrowser<'a> {
        TargetFileBrowser {
            files: HashMap::new(),
            base_path: String::from(base_path),
            preserve_path,
            on_progress
        }
    }

    pub fn get_base_path(&self) -> String {
        return self.base_path.clone();
    }

    pub fn init(&mut self) {
        self.parse(&self.get_base_path());
    }

    fn add_to_file(&mut self, hash: &String, path_source: &PathBuf, path_dest: &String) {
        let source = path_source.clone().into_os_string().into_string().unwrap().clone().replace("\\", "/");
        debug!("source exists {}", std::path::Path::new(source.as_str()).exists());
        let dest = path_dest.clone().replace("\\", "/");
        let (dir, _file) = dest.split_at(dest.rfind("/").unwrap());
        let res_dir_creation = std::fs::create_dir_all(dir);
        if res_dir_creation.is_err() {
           error!("Impossible de créer le dossier, {:?}", res_dir_creation.err());
        }
        self.files.insert(
            hash.clone(),
            dest.clone()
        );
        let copy = std::fs::copy(&source, &dest);
        if copy.is_err() {
            error!("Erreur durant la copie de {:?} vers {:?} {}", source, dest, copy.err().unwrap());
        }
    }

    fn is_in_files(&self, hash: &String) -> bool {
        return self.files.contains_key(hash);
    }
}

impl<'a> FileBrowser for TargetFileBrowser<'a> {
    fn on_file_entry(&mut self, path: &PathBuf) {
        let hash = self.get_hash(&path);
        self.files.insert(hash.clone(), path.clone().into_os_string().into_string().unwrap());
        (self.on_progress)(path);
    }
}

struct SourceFileBrowser<'b, 'c> {
    base_path: String,
    target: &'b mut TargetFileBrowser<'c>,
    on_progress: &'b dyn Fn(&PathBuf)
}

impl<'b, 'c> SourceFileBrowser<'b, 'c> {
    pub fn new(base_path: &str, target: &'b mut TargetFileBrowser<'c>, on_progress: &'b dyn Fn(&PathBuf)) -> SourceFileBrowser<'b, 'c> {
        SourceFileBrowser {
            base_path: String::from(base_path.clone()),
            target,
            on_progress
        }
    }

    pub fn init(&mut self) {
        self.parse(&self.base_path.clone());
    }
}

impl<'b, 'c> FileBrowser for SourceFileBrowser<'b, 'c> {
    fn on_file_entry(&mut self, path: &PathBuf) {
        let hash = self.get_hash(&path);
        if self.target.is_in_files(&hash) {
            debug!("File with hash {:?} already exists", hash);
            info!("File {:?} already exists in destination.", path);
            (self.on_progress)(path);
            return;
        }
        let mut original_dest_path = self.target.get_base_path();
        if self.target.preserve_path {
            original_dest_path = original_dest_path + path.clone().into_os_string().into_string().unwrap().split_off(self.base_path.len()).as_str();
        } else {
            let temp_str = path.clone().into_os_string().into_string().unwrap();
            let v: Vec<&str> = temp_str.rsplit(|c| c == '/' || c == '\\').collect();
            original_dest_path = original_dest_path + "/" + v[0];
        }

        let mut final_dest_path = original_dest_path.clone();
        let mut i = 1;
        while std::path::Path::new(final_dest_path.as_str()).exists() {
            let last_point_index = original_dest_path.rfind(".");
            let mut str_copy = original_dest_path.clone();
            let mut extension = String::new();
            if last_point_index.is_some() {
                extension = str_copy.split_off(last_point_index.unwrap());
            }
            final_dest_path = str_copy.clone() + format!(" ({:?})", i).as_str();
            if extension != "" {
                final_dest_path = final_dest_path + extension.as_str();
            }
            debug!("final path {:?} original: {:?} extension {:?}", final_dest_path, original_dest_path, extension);
            i = i + 1;
        }
        debug!("copying file {:?} path {:?}, target: {:?}", hash, path, final_dest_path);
        debug!("Base path {:?} current path {:?}", self.base_path, path);
        self.target.add_to_file(&hash, path, &final_dest_path);
        (self.on_progress)(path);
    }
}

struct FileCounter {
    count: i32
}

impl FileCounter {
    pub fn new() -> FileCounter {
        FileCounter {
            count: 0
        }
    }

    pub fn count(&mut self, base_path: &String) -> i32 {
        self.count = 0;
        self.parse(base_path);
        return self.count;
    }
}

impl FileBrowser for FileCounter {
    fn on_file_entry(&mut self, _path: &PathBuf) {
        self.count = self.count + 1;
    }
}

pub fn get_total_files(sources: &Vec::<String>) -> i32 {
    let mut file_counter = FileCounter::new();
    let mut total = 0;
    for source in sources {
        total += file_counter.count(&source);
    }
    return total;
}

pub fn start_copy(
    target: &str,
    sources: &Vec::<String>,
    preserve_path: bool,
    on_progress_target: &dyn Fn(&PathBuf),
    on_progress_source: &dyn Fn(&PathBuf)
) {
    let mut target = TargetFileBrowser::new(&target, preserve_path, &on_progress_target);
    target.init();

    for source in sources {
        let mut source_file_browser = SourceFileBrowser::new(&source, &mut target, &on_progress_source);
        source_file_browser.init();
    }
}