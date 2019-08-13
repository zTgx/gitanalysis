use crate::core::engine::Engine;
use crate::http::path::Path;
use serde_json::{Value, Result};

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

    pub fn auth(&mut self) -> Result<Value> {
        let path = Path::new().and(&"users".to_owned()).and(&self.username).ok();

        self.engine.engine.username(&self.username).unwrap();
        self.engine.engine.password(&self.password).unwrap();

        self.engine.get(&path)
    }
}
