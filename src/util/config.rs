use crate::util::constant::{HOST};

pub struct Config {}
impl Config {
    pub fn host() -> String {
        HOST.to_owned()
    }

    pub fn get_url(path: &String) -> String {
        Config::host().to_owned() + path.as_str()
    }
}
