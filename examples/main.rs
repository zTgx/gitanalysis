// extern crate curl;

// use curl::easy::{Easy, List};
// use serde_json::{Value};

extern crate gitapi_rs as Api;
use Api::GitApi;
fn main() {

    println!("name: {}", GitApi::new().get_profile("zTgx")["login"]);

    // let mut handle = Easy::new();
    //
    // let mut list = List::new();
    // list.append("User-Agent: GITAPI-RS").unwrap();
    // handle.http_headers(list).unwrap();
    //
    // handle.url("https://api.github.com/users/zTgx").unwrap();
    //
    // let mut buf = Vec::new();
    // {
    //     let mut transfer = handle.transfer();
    //     transfer.write_function(|data| {
    //         buf.extend_from_slice(data);
    //         Ok(data.len())
    //     }).unwrap();
    //     transfer.perform().unwrap();
    // }
    //
    // // println!("{:#?}", String::from_utf8(buf));
    // let info: Value = serde_json::from_slice(&buf).unwrap();
    // println!("{:#?}", info["login"]);
}
