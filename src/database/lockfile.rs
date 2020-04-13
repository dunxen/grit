use std::{
    error::Error,
    fmt::{self, Display},
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
};

#[derive(Debug)]
pub enum LockError {
    MissingParent,
    NoPermission,
    StaleLock(PathBuf),
}

impl Display for LockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LockError::StaleLock(path) => write!(f, "Not holding lock on file: {}", path.display()),
            _ => write!(f, "{}", self),
        }
    }
}

impl Error for LockError {}

pub struct Lockfile {
    file_path: PathBuf,
    lock_path: PathBuf,
    lock: Option<File>,
}

impl Lockfile {
    pub fn new(path: &PathBuf) -> Self {
        let mut lock_path = path.to_owned();
        lock_path.set_extension(".lock");

        Lockfile {
            lock_path,
            file_path: path.to_owned(),
            lock: None,
        }
    }

    pub fn hold_for_update(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        if self.lock.is_none() {
            let mut options = fs::OpenOptions::new();
            options.read(true).write(true).create_new(true);
            match options.open(&self.lock_path) {
                Ok(file) => {
                    self.lock = Some(file);
                    return Ok(true);
                }
                Err(error) => match error.kind() {
                    io::ErrorKind::AlreadyExists => return Ok(false),
                    io::ErrorKind::NotFound => return Err(LockError::MissingParent.into()),
                    io::ErrorKind::PermissionDenied => return Err(LockError::NoPermission.into()),
                    _ => return Err(error.into()),
                },
            }
        }

        Ok(false)
    }

    pub fn write(&mut self, content: &str) -> Result<usize, Box<dyn std::error::Error>> {
        match &mut self.lock {
            Some(lock) => lock.write(content.as_bytes()).map_err(|error| error.into()),
            None => Err(LockError::StaleLock((&self.lock_path).clone()).into()),
        }
    }

    pub fn commit(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match &mut self.lock {
            Some(lock) => {
                lock.sync_all()?;
                fs::rename(&self.lock_path, &self.file_path)?;
                self.lock = None;
                Ok(())
            }
            None => Err(LockError::StaleLock((&self.lock_path).clone()).into()),
        }
    }
}
