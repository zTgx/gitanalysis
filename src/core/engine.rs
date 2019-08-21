use curl::easy::{Easy, Form};
use serde_json::{Value, Result, json};
use crate::util::config::Config;
use crate::http::header::Header;
use std::io::Read;

macro_rules! t {
    ($e:expr) => {
        match $e {
            Ok(e) => e,
            Err(e) => panic!("{} failed with {:?}", stringify!($e), e),
        }
    };
}

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
    pub fn post(&mut self, path: &String, body: Value) {
        self.headers();
        self.engine.url(&path).unwrap();

        println!("path: {}", &path);

        let s = String::from( body.to_string() );
        self.engine.post(true).unwrap();
        self.engine.post_field_size(25u64).unwrap();

        self.engine.read_function(move |buf| {
            Ok(s.as_bytes().read(buf).unwrap_or(0))
        }).unwrap();
        self.engine.perform().unwrap();
    }

    pub fn get(&mut self, path: &String) -> Result<Value> {
        self.headers();
        self.engine.url(&path).unwrap();

        //post
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
