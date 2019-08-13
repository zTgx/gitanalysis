use curl::easy::{Easy};
use serde_json::{Value, Result};
use crate::util::config::Config;
use crate::http::header::Header;

pub struct Engine {
    pub host: String,
    pub engine: Easy,
}
impl Engine {
    pub fn new() -> Self {
        let handle = Easy::new();
        Engine {
            host: Config::host(),
            engine: handle,
        }
    }

    pub fn headers(&mut self) {
        self.engine.http_headers(Header::get()).unwrap();
    }
}

impl Engine {
    pub fn get(&mut self, path: &String) -> Result<Value> {
        self.headers();

        self.engine.url(&path).unwrap();

        let mut buf = Vec::new();
        {
            let mut transfer = self.engine.transfer();
            transfer.write_function(|data| {
                buf.extend_from_slice(data);
                Ok(data.len())
            }).unwrap();
            transfer.perform().unwrap();
        }

        serde_json::from_slice(&buf)
    }
}
