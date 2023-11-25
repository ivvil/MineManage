use std::process::{Child, Command, Stdio};

fn create_child_process(bin : &str, bin_args: &[&str]) -> Child {
    let mut process = Command::new(bin)
        .args(bin_args) // Pass as arguments -jar and the global location of the binary
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .spawn()
        .expect(&*[ bin, "has failed"].join("").to_string());
    process
}