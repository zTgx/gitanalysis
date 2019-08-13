extern crate gitapi_rs as Api;
use Api::{Engine, BasicAuth};
fn main() {
    // let path = Path::new().and("users").and("zTgx").ok();
    // println!("profile: \n{:?}", GitApi::new().get(&path));

    let mut engine = Engine::new();
    let v: Value = BasicAuth::new(&mut engine).username("username".to_owned())
                                              .password("password".to_owned())
                                              .auth();
    println!("basic: {:?}", );
}
