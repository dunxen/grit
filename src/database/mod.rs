use std::fs;
use std::path::PathBuf;

use bytes::{BufMut, BytesMut};
use deflate;
use rand::seq::SliceRandom;
use sha1::Sha1;

pub mod author;
pub mod refs;

pub use author::Author;
pub use refs::Refs;

mod hex;

static TEMP_CHAR_SET: [char; 62] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9',
];

pub trait Object {
    fn get_oid(&self) -> String;
    fn set_oid(&mut self, oid: String);
    fn get_type(&self) -> String;
    fn bytes(&mut self) -> Vec<u8>;
}

pub struct Database {
    path: std::path::PathBuf,
}

impl Database {
    pub fn new(path: &PathBuf) -> Self {
        Database {
            path: path.to_owned(),
        }
    }

    pub fn store<T: Object>(&self, object: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let content = object.bytes();

        let hash = Sha1::from(&content);

        object.set_oid(hash.digest().to_string());
        self.write_object(&object.get_oid(), content.as_slice())
    }

    pub fn write_object(
        &self,
        oid: &str,
        content: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let dirname = self.path.join(&oid[0..2]);
        let object_path = &dirname.join(&oid[2..]);
        let temp_path = PathBuf::from(&dirname).join(Database::generate_temp_name());

        let compressed =
            deflate::deflate_bytes_zlib_conf(content, deflate::CompressionOptions::fast());
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
}

impl Object for Blob {
    fn get_oid(&self) -> String {
        self.oid.to_owned()
    }

    fn set_oid(&mut self, oid: String) {
        self.oid = oid;
    }

    fn get_type(&self) -> String {
        String::from("blob")
    }

    fn bytes(&mut self) -> Vec<u8> {
        let mut content_buf = BytesMut::new();
        content_buf.put(self.get_type().as_bytes());
        content_buf.put(&b" "[..]);
        content_buf.put(self.data.len().to_string().as_bytes());
        content_buf.put(&b"\0"[..]);
        content_buf.put(self.data.as_bytes());

        content_buf.to_vec()
    }
}

#[derive(Clone)]
pub struct Entry {
    name: String,
    oid: String,
}

impl Entry {
    pub fn new(name: &str, oid: &str) -> Self {
        Entry {
            name: name.to_owned(),
            oid: oid.to_owned(),
        }
    }
}

pub struct Tree {
    oid: String,
    entries: Vec<Entry>,
}

impl Tree {
    const MODE: &'static str = "100644";
    pub fn new(entries: Vec<Entry>) -> Self {
        Tree {
            entries,
            oid: String::from(""),
        }
    }
}

impl Object for Tree {
    fn get_oid(&self) -> String {
        self.oid.to_owned()
    }

    fn set_oid(&mut self, oid: String) {
        self.oid = oid;
    }

    fn get_type(&self) -> String {
        String::from("tree")
    }

    fn bytes(&mut self) -> Vec<u8> {
        self.entries.sort_by_key(|x| x.name.clone());
        let mut buf = BytesMut::new();

        for entry in self.entries.iter() {
            buf.put(Tree::MODE.as_bytes());
            buf.put(&b" "[..]);
            buf.put(entry.name.as_bytes());
            buf.put(&b"\0"[..]);
            buf.put(hex::decode_hex(&entry.oid).unwrap().as_slice());
        }

        let mut content_buf = BytesMut::new();
        content_buf.put(self.get_type().as_bytes());
        content_buf.put(&b" "[..]);
        content_buf.put(buf.len().to_string().as_bytes());
        content_buf.put(&b"\0"[..]);
        content_buf.put(buf);

        content_buf.to_vec()
    }
}

pub struct Commit {
    oid: String,
    parent: Option<String>,
    tree_oid: String,
    author: Author,
    message: String,
}

impl Commit {
    pub fn new(parent: &Option<String>, tree_oid: &str, author: Author, message: &str) -> Self {
        Commit {
            author,
            parent: parent.to_owned(),
            oid: String::from(""),
            tree_oid: tree_oid.to_owned(),
            message: message.to_owned(),
        }
    }
}

impl Object for Commit {
    fn get_oid(&self) -> String {
        self.oid.to_owned()
    }

    fn set_oid(&mut self, oid: String) {
        self.oid = oid
    }

    fn get_type(&self) -> String {
        String::from("commit")
    }

    fn bytes(&mut self) -> Vec<u8> {
        let mut buf = BytesMut::new();
        buf.put(&b"tree "[..]);
        buf.put(self.tree_oid.as_bytes());

        if let Some(parent) = &self.parent {
            buf.put(&b"\nparent "[..]);
            buf.put(parent.as_bytes());
        }

        buf.put(&b"\nauthor "[..]);
        buf.put(self.author.bytes().as_slice());
        buf.put(&b"\ncommitter "[..]);
        buf.put(self.author.bytes().as_slice());
        buf.put(&b"\n\n"[..]);
        buf.put(self.message.as_bytes());

        let mut content_buf = BytesMut::new();
        content_buf.put(self.get_type().as_bytes());
        content_buf.put(&b" "[..]);
        content_buf.put(buf.len().to_string().as_bytes());
        content_buf.put(&b"\0"[..]);
        content_buf.put(buf);

        content_buf.to_vec()
    }
}
