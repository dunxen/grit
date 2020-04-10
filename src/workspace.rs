use std;

pub struct Workspace {
    path: std::path::PathBuf,
}

impl Workspace {
    pub fn new(path: &std::path::PathBuf) -> Self {
        Workspace {
            path: path.to_owned(),
        }
    }

    pub fn list_files(&self) -> Result<std::fs::ReadDir, Box<dyn std::error::Error>> {
        std::fs::read_dir(&self.path).map_err(|error| error.into())
    }

    pub fn read_file(path: &std::path::PathBuf) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        std::fs::read(path).map_err(|error| error.into())
    }
}
