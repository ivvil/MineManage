use std::process::{Child, Command, Stdio};
use std::io::{self, Write};
use serde::{Serialize, Deserialize};
use confy;
use std::string::String;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    version: u8,
    java_location : String,
    bin_location : String,
    bin_name : String,
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for Config {
    fn default() -> Self { Self { version: 0, java_location: "java".into(), bin_location: "".into(), bin_name: "".into()} }
}

fn main() -> Result<(), confy::ConfyError> {
    let cfg: Config = confy::load("mine_manage", "config")?; // Load config

    println!("{:?}", cfg);

    let process = create_child_process(cfg.java_location, &["-jar", [cfg.bin_location, cfg.bin_name].join("").as_str()]);

    /*let mut stdin = process.stdin.take().expect("Failed to open stdin");*/
    let stdout = process.wait_with_output().expect("Failed to read stdout");


    print!("{}", String::from_utf8_lossy(&stdout.stdout));


    Ok(())
}

fn create_child_process(bin : String, bin_args: &[&str]) -> Child {
    let mut process = Command::new(bin)
        .args(bin_args) // Pass as arguments -jar and the global location of the binary
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .spawn()
        .expect("java has failed");
    process
}

