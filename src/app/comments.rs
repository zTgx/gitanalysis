use crate::core::engine::Engine;
use crate::http::path::Path;
use serde_json::{Value, Result};

pub struct Comments <'a> {
    pub engine: &'a mut Engine,
    pub username: String,
    pub repo: String,
    pub issue_number: usize,
    pub body: String,
}
impl <'a> Comments <'a> {
    pub fn new(engine: &'a mut Engine) -> Self {
        Comments {
            engine: engine,
            username: "".to_owned(),
            repo: "".to_owned(),
            issue_number: 0,
            body: "".to_owned(),
        }
    }

    pub fn username(&mut self, username: String) -> &mut Self {
        self.username = username;

        self
    }

    pub fn repo(&mut self, repo: String) -> &mut Self {
        self.repo = repo;

        self
    }

    pub fn issue_number(&mut self, issue_number: usize) -> &mut Self {
        self.issue_number = issue_number;

        self
    }

    pub fn body(&mut self, body: String) -> &mut Self {
        self.body = body;

        self
    }

    pub fn submit(&mut self) {
        let path = Path::new().slash(&"repos".to_owned())
                              .slash(&self.username)
                              .slash(&self.repo)
                              .slash(&"issues".to_owned())
                              .slash(&self.issue_number.to_string())
                              .slash(&"comments".to_owned())
                              .ok();

        let data = r#"
          {
              "body": "John Doe"
          }"#;

        // Parse the string of data into serde_json::Value.
        let v: Value = serde_json::from_str(data).unwrap();
        self.engine.post(&path, v);
    }

    pub fn list(&mut self) -> Result<Value> {
        if self.issue_number != 0 {
            let path = Path::new().slash(&"repos".to_owned())
                                  .slash(&self.username)
                                  .slash(&self.repo)
                                  .slash(&"issues".to_owned())
                                  .slash(&self.issue_number.to_string())
                                  .slash(&"comments".to_owned())
                                  .ok();

            //List comments on an issue
            self.engine.get(&path)
      } else {
          let path = Path::new().slash(&"repos".to_owned())
                                .slash(&self.username)
                                .slash(&self.repo)
                                .slash(&"issues".to_owned())
                                .slash(&"comments".to_owned())
                                .ok();

          //List comments in a repository
          self.engine.get(&path)
      }
    }

    //GET /repos/:owner/:repo/issues/comments/:comment_id
    pub fn list_comment_id(&mut self, idx: u64) -> Result<Value> {
        let path = Path::new().slash(&"repos".to_owned())
                              .slash(&self.username)
                              .slash(&self.repo)
                              .slash(&"issues".to_owned())
                              .slash(&"comments".to_owned())
                              .slash(&idx.to_string())
                              .ok();

        //List comments on an issue
        self.engine.get(&path)
    }
}
