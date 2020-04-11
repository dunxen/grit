use std;

pub struct Workspace {
    path: std::path::PathBuf,
}

type EntryPredicate = fn(&std::io::Result<std::fs::DirEntry>) -> bool;

impl Workspace {
    const IGNORE: [&'static str; 1] = [".git"];

    pub fn new(path: &std::path::PathBuf) -> Self {
        Workspace {
            path: path.to_owned(),
        }
    }

    pub fn list_files(
        &self,
    ) -> Result<std::iter::Filter<std::fs::ReadDir, EntryPredicate>, Box<dyn std::error::Error>>
    {
        let ignore: EntryPredicate = |entry| {
            if let Ok(entry) = entry {
                return !Workspace::IGNORE
                    .to_vec()
                    .contains(&entry.file_name().to_str().unwrap());
            }

            false
        };

        std::fs::read_dir(&self.path)
            .map(|x| x.filter(ignore))
            .map_err(|error| error.into())
    }

    pub fn read_file(path: &std::path::PathBuf) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        std::fs::read(path).map_err(|error| error.into())
    }
}
