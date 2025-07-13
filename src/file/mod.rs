use std::fs::{read_dir, DirEntry};

pub fn entries() -> Vec<DirEntry> {
    let mut entries = Vec::new();
    let paths = read_dir("/home/tzamsk/MyNotes/ETS/").unwrap();
    for path in paths {
        entries.push(path.expect("Entry doesnt exist"));
    }

    entries.into_iter().collect()
}

pub fn count_entries() -> u16 {
    return entries().iter().count().try_into().unwrap();
}
