extern crate gitapi_rs as Api;
use Api::{GitApi, Path};
fn main() {
    let mut path = Path::new();
    path.and("users").and("zTgx");
    println!("name: {}", GitApi::new().get(&mut path)["login"]);
}
