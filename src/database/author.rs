use bytes::{BufMut, BytesMut};
use time;

pub struct Author {
    name: String,
    email: String,
    time: time::OffsetDateTime,
}

impl Author {
    pub fn new(name: &str, email: &str) -> Self {
        Author {
            name: name.to_owned(),
            email: email.to_owned(),
            time: time::OffsetDateTime::now_local(),
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut buf = BytesMut::new();
        let timestamp = format!(
            "{} {}",
            self.time.timestamp().to_string(),
            self.time.format("%z")
        );
        buf.put(self.name.as_bytes());
        buf.put(&b" <"[..]);
        buf.put(self.email.as_bytes());
        buf.put(&b"> "[..]);
        buf.put(timestamp.as_bytes());
        buf.to_vec()
    }
}
