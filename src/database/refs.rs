use std::{error::Error, fmt, fmt::Display, fs, path::PathBuf};

use super::lockfile::Lockfile;

#[derive(Debug)]
struct LockDenied(PathBuf);

impl Display for LockDenied {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Not holding lock on file: {}", self.0.display())
    }
}

impl Error for LockDenied {}

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

    pub fn update_head(&self, oid: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut lockfile = Lockfile::new(&self.head_path());

        match lockfile.hold_for_update() {
            Ok(true) => {
                lockfile.write(oid)?;
                lockfile.write("\n")?;
                lockfile.commit()
            }
            Ok(false) => Err(LockDenied(self.head_path()).into()),
            Err(error) => Err(error),
        }
    }

    pub fn read_head(&self) -> Option<String> {
        if let Ok(content) = fs::read(self.head_path()) {
            return Some(String::from_utf8(content).unwrap().trim().to_owned());
        }

        None
    }
}
