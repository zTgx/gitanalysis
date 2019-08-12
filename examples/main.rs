extern crate gitapi_rs as Api;
use Api::GitApi;
fn main() {
    println!("name: {}", GitApi::new().get_profile("/users/zTgx")["login"]);
}
