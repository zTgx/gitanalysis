extern crate gitapi_rs as Api;
use Api::core::engine::Engine;
use Api::app::profile::Profile;

fn main() {
    let mut engine = Engine::new();
    let profile = Profile::new(&mut engine).username("username".to_owned()).get();
    println!("profile: {:?}", profile);
}
