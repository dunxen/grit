use std::{fs, path::PathBuf};

pub struct Refs {
    path: PathBuf,
}

impl Refs {
    pub fn new(path: &PathBuf) -> Self {
        Refs {
            path: path.to_owned(),
        }
    }

    pub fn head_path(&self) -> PathBuf {
        self.path.join("HEAD")
    }

    pub fn update_head(&self, oid: &str) -> std::io::Result<()> {
        fs::write(&self.head_path(), oid)
    }

    pub fn read_head(&self) -> Option<String> {
        if let Ok(content) = fs::read(self.head_path()) {
            return Some(String::from_utf8(content).unwrap().trim().to_owned());
        }

        None
    }
}
