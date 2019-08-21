extern crate gitapi_rs as Api;
use Api::core::engine::Engine;
use Api::app::comments::Comments;
use Api::auth::basic::BasicAuth;

fn main() {
    let mut engine = Engine::new();

    let v = BasicAuth::new(&mut engine).username("beautifularea".to_owned())
                                      .password("password".to_owned())
                                      .auth();
    // println!("v: {}", v.unwrap());

    let _list = Comments::new(&mut engine).username("zTgx".to_owned())
                                            .repo("gitapi-rs".to_owned())
                                            .issue_number(1)
                                            .body("Crate comments from API".to_owned())
                                            .submit();
}

curl -s -u "beautifularea" -X POST -d '{"body":"YourMessagetoComment"}' https://api.github.com/repos/zTgx/gitapi-rs/issues/1/comments
