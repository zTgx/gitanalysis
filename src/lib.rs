
use serde_json::{Value};
extern crate curl;
use curl::easy::{Easy, List};

static HOST: &'static str = "https://api.github.com";

pub struct GitApi {
    pub engine: Easy,
}
impl GitApi {
    pub fn new() -> Self {
        let mut handle = Easy::new();
        GitApi {
            engine: handle,
        }
    }
}

impl GitApi {
    pub fn get_profile(&mut self, name: &str) -> Value {
        let mut list = List::new();
        list.append("User-Agent: GITAPI-RS").unwrap();
        self.engine.http_headers(list).unwrap();

        self.engine.url("https://api.github.com/users/zTgx").unwrap();

        let mut buf = Vec::new();
        {
            let mut transfer = self.engine.transfer();
            transfer.write_function(|data| {
                buf.extend_from_slice(data);
                Ok(data.len())
            }).unwrap();
            transfer.perform().unwrap();
        }

        // println!("{:#?}", String::from_utf8(buf));
        let info: Value = serde_json::from_slice(&buf).unwrap();
        // println!("{:#?}", info["login"]);

        info
    }
}
