
use serde_json::{Value};
extern crate curl;
use curl::easy::{Easy, List};
use curl::Error;

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
    pub fn host() -> String {
        HOST.to_owned()
    }

    pub fn get_url(path: &String) -> String {
        Config::host().to_owned() + path.as_str()
    }
}

pub struct Path{
    pub host: String,
}
impl Path {
    pub fn new() -> Self {
        Path {
            host: Config::host(),
        }
    }

    pub fn and(&mut self, p: &String) -> &mut Self {
        self.host = self.host.to_owned() + "/" + p.as_str();

        self
    }

    pub fn ok(&self) -> String {
        self.host.to_owned()
    }
}

pub enum AuthenType {
    Basic(String),
    Oauth2(String),
}

pub struct BasicAuth <'a> {
    pub engine: &'a mut Engine,
    pub username: String,
    pub password: String,
}
impl <'a> BasicAuth <'a> {
    pub fn new(engine: &'a mut Engine) -> Self {
        BasicAuth {
            engine: engine,
            username: "".to_owned(),
            password: "".to_owned(),
        }
    }

    pub fn username(&mut self, username: String) -> &mut Self {
        self.username = username;

        self
    }

    pub fn password(&mut self, password: String) -> &mut Self {
        self.password = password;

        self
    }

    pub fn auth(&mut self) -> Result<Value, String> {
        let path = Path::new().and(&"users".to_owned()).and(&self.username).ok();

        self.engine.username(&self.username).unwrap();
        self.engine.password(&self.password).unwrap();

        Ok( self.engine.get(&path) )
    }
}

pub struct Profile <'a> {
    pub engine: &'a mut Engine,
    pub username: String,
}
impl <'a> Profile <'a> {
    pub fn new(engine: &'a mut Engine) -> Self {
        Profile {
            engine: engine,
            username: "".to_owned(),
        }
    }

    pub fn username(&mut self, username: String) -> &mut Self {
        self.username = username;

        self
    }

    pub fn get(&mut self) -> Result<Value, String> {
        let path = Path::new().and(&"users".to_owned()).and(&self.username).ok();

        Ok( self.engine.get(&path) )
    }
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

    pub fn username(&mut self, username: &String) -> Result<(), Error> {
        self.engine.username(&username)
    }

    pub fn password(&mut self, password: &String) -> Result<(), Error> {
        self.engine.password(&password)
    }
}

impl Engine {
    pub fn get(&mut self, path: &String) -> Value {
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

        let info: Value = serde_json::from_slice(&buf).unwrap();

        info
    }
}
