use crate::util::config::Config;

pub struct Path{
    pub host: String,
}
impl Path {
    pub fn new() -> Self {
        Path {
            host: Config::host(),
        }
    }

    pub fn slash(&mut self, p: &String) -> &mut Self {
        self.host = self.host.to_owned() + "/" + p.as_str();

        self
    }

    pub fn ok(&self) -> String {
        self.host.to_owned()
    }
}
