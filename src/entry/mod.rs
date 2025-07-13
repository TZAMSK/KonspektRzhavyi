use std::{
    fs::{read_dir, DirEntry},
    path::{Path, PathBuf},
};

pub struct Entry {
    pub entries: Vec<DirEntry>,
    pub current_path: PathBuf,
    pub pointing_path: PathBuf,
}

impl Default for Entry {
    fn default() -> Self {
        Self {
            entries: Self::read_entries("/home/tzamsk/MyNotes/"),
            current_path: PathBuf::from("/home/tzamsk/MyNotes/"),
            pointing_path: PathBuf::new(),
        }
    }
}

impl Entry {
    pub fn read_entries(path: impl Into<PathBuf> + AsRef<Path>) -> Vec<DirEntry> {
        read_dir(path)
            .unwrap()
            .map(|entry| entry.expect("Entry doesn't exist"))
            .collect()
    }

    pub fn enter_directory(&mut self, dir: &Path) {
        self.entries = Self::read_entries(dir);
        self.current_path = dir.to_path_buf();
    }

    pub fn count_entries(&self) -> u16 {
        return self.entries.len() as u16;
    }
}
