
use serde_json::{Value};
extern crate curl;
use curl::easy::{Easy, List};

static HOST: &'static str = "https://api.github.com";

pub struct Header {}
impl Header {
    pub fn get() -> List {
        let mut list = List::new();
        list.append("User-Agent: GITAPI-RS").unwrap();

        list
    }
}

pub struct Config {}
impl Config {
    pub fn host() -> &'static str {
        HOST
    }

    pub fn get_url(path: &'static str) -> String {
        Config::host().to_owned() + path
    }
}

pub struct GitApi {
    pub host: &'static str,
    pub engine: Easy,
}
impl GitApi {
    pub fn new() -> Self {
        let handle = Easy::new();
        GitApi {
            host: Config::host(),
            engine: handle,
        }
    }

    pub fn headers(&mut self) {
        self.engine.http_headers(Header::get()).unwrap();
    }
}

impl GitApi {
    pub fn get_profile(&mut self, path: &'static str) -> Value {
        self.headers();

        self.engine.url(&Config::get_url(path)).unwrap();

        let mut buf = Vec::new();
        {
            let mut transfer = self.engine.transfer();
            transfer.write_function(|data| {
                buf.extend_from_slice(data);
                Ok(data.len())
            }).unwrap();
            transfer.perform().unwrap();
        }

        let info: Value = serde_json::from_slice(&buf).unwrap();

        info
    }
}
