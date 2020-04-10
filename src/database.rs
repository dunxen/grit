use std::fs;
use std::path::PathBuf;

use deflate;
use rand::seq::SliceRandom;
use sha1::Sha1;

static TEMP_CHAR_SET: [char; 62] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9',
];

pub struct Database {
    path: std::path::PathBuf,
}

impl Database {
    pub fn new(path: &PathBuf) -> Self {
        Database {
            path: path.to_owned(),
        }
    }

    pub fn store(&self, blob: &mut Blob) -> Result<(), Box<dyn std::error::Error>> {
        let string = blob.to_string();
        let content = format!("{} {}\0{}", blob.get_type(), string.len(), string);

        let hash = Sha1::from(&content);

        blob.set_oid(hash.digest().to_string());
        self.write_object(&blob.oid, &content)
    }

    pub fn write_object(&self, oid: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let dirname = self.path.join(&oid[0..2]);
        let object_path = &dirname.join(&oid[2..]);
        let temp_path = PathBuf::from(&dirname).join(Database::generate_temp_name());

        let compressed =
            deflate::deflate_bytes_conf(content.as_bytes(), deflate::CompressionOptions::fast());
        if let Err(error) = fs::write(&temp_path, &compressed) {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    fs::create_dir(PathBuf::from(dirname))?;
                    fs::write(&temp_path, compressed)?;
                }
                _ => return Err(error.into()),
            }
        }
        fs::rename(temp_path, object_path).map_err(|error| error.into())
    }

    fn generate_temp_name() -> String {
        let sample: String = TEMP_CHAR_SET
            .choose_multiple(&mut rand::thread_rng(), 6)
            .collect();
        format!("tmp_obj_{}", sample)
    }
}

pub struct Blob {
    oid: String,
    data: String,
}

impl Blob {
    pub fn new(data: &str) -> Self {
        Blob {
            data: data.to_owned(),
            oid: String::from(""),
        }
    }

    pub fn get_type(&self) -> String {
        String::from("blob")
    }

    pub fn set_oid(&mut self, oid: String) {
        self.oid = oid;
    }
}

impl std::fmt::Display for Blob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}
