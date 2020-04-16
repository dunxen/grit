use std::{os::unix::fs::PermissionsExt, path::Path};

use walkdir::{DirEntry, WalkDir};

pub struct Workspace {
    path: std::path::PathBuf,
}

impl Workspace {
    const IGNORE: [&'static str; 1] = [".git"];

    pub fn new(path: &std::path::PathBuf) -> Self {
        Workspace {
            path: path.to_owned(),
        }
    }

    pub fn list_files(&self) -> impl Iterator<Item = DirEntry> {
        WalkDir::new(&self.path)
            .into_iter()
            .filter_entry(|e| {
                Workspace::IGNORE
                    .iter()
                    .any(|a| a != &e.file_name().to_str().unwrap())
            })
            .filter_map(|e| e.ok())
            .filter(|e| e.metadata().unwrap().is_file())
    }

    pub fn read_file(path: &Path) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        std::fs::read(path).map_err(|error| error.into())
    }

    pub fn stat_file(path: &Path) -> Result<u32, Box<dyn std::error::Error>> {
        let meta = std::fs::metadata(path)?;
        Ok(meta.permissions().mode())
    }
}
