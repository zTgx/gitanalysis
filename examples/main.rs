extern crate gitapi_rs;
use gitapi_rs::Git;

fn main() {
    let all: String = Git::all_branch();
    println!("all branch: {}", all);
}
