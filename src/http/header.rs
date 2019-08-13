use curl::easy::{List};

pub struct Header {}
impl Header {
    pub fn get() -> List {
        let mut list = List::new();
        list.append("User-Agent: GITAPI-RS").unwrap();

        list
    }
}
