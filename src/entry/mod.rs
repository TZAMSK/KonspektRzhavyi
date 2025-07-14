use std::{
    fs::{self, read_dir, DirEntry, File},
    path::{Path, PathBuf},
};

pub struct Entry {
    pub entries: Vec<DirEntry>,
    pub current_path: PathBuf,
    pub pointing_path: PathBuf,
    pub current_file: Option<File>,
}

impl Entry {
    pub fn init() -> Self {
        Self {
            entries: Self::read_entries("/home/tzamsk/MyNotes/"),
            current_path: PathBuf::from("/home/tzamsk/MyNotes/"),
            pointing_path: PathBuf::new(),
            current_file: None,
        }
    }

    pub fn read_entries(path: impl Into<PathBuf> + AsRef<Path>) -> Vec<DirEntry> {
        read_dir(path)
            .unwrap()
            .map(|entry| entry.expect("Entry doesn't exist"))
            .collect()
    }

    pub fn enter_directory(&mut self, dir: &Path) {
        if dir.is_dir() {
            self.entries = Self::read_entries(dir);
            self.current_path = dir.to_path_buf();
        } else if dir.is_file() {
            self.current_path = dir.to_path_buf();
            self.current_file = Some(File::open(dir).expect("File doesn't exist"));
        }
    }

    pub fn read_file(&self) -> String {
        match self.current_file {
            Some(_) => fs::read_to_string(&self.current_path).expect("Fle doesn't exist"),
            None => "Choose a file".to_string(),
        }
    }

    pub fn count_entries(&self) -> u16 {
        return self.entries.len() as u16;
    }
}
