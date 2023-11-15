use std::process::{Command, Stdio};
use std::io::{self, Write};
use serde::{Serialize, Deserialize};
use confy;

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
    let cfg: Config = confy::load("MineManage", "config.toml")?; // Load config

    println!("{:?}", cfg);

    let process = Command::new(cfg.java_location)
            .args(["-jar", [cfg.bin_location, cfg.bin_name].join("").as_str()]) // Pass as arguments -jar and the global location of the binary
            .stdin(Stdio::inherit())
            .stdout(Stdio::piped())
            .spawn()
            .expect("java has failed");
    Ok(())
}
