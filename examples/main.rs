extern crate gitapi_rs as Api;
use Api::{GitApi, Path};
fn main() {
    let path = Path::new().and("users").and("zTgx").ok();
    println!("profile: \n{:?}", GitApi::new().get(&path));
}
