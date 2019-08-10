use std::process::Command;

pub struct Git {}
impl Git {
    pub fn all_branch() {
        let output = Command::new("git")
                         .arg("branch")
                         .arg("-a")
                         .output()
                         .expect("Failed to execute command");
         let output = output.stdout.as_slice().to_vec();

         let ret = String::from_utf8(output).unwrap();
         println!("ret = {}", ret);
    }
}
