use crate::core::engine::Engine;
use crate::http::path::Path;
use serde_json::{Value, Result};

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

    pub fn get(&mut self) -> Result<Value> {
        let path = Path::new().slash(&"users".to_owned()).slash(&self.username).ok();

        self.engine.get(&path)
    }
}
