use std::{fs, path};
use std::fs::ReadDir;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug)]
struct Config {
    config_version : u8,
    default_java_location : String,
    default_bin_location : String,
    default_bin_name : String,
    default_custom_jvm_arguments : String,
    instances_directory : PathBuf
}


impl Default for Config {
    fn default() -> Self {

        Self {
        config_version: 1,
        default_java_location: "java".into(),
        default_bin_location: "".into(),
        default_bin_name: "".into(),
        default_custom_jvm_arguments: "".into(),
        instances_directory: super::utils::INSTANCES_DIRECTORY
        } }
}

fn init_daemon() {
    let cfg : Config = confy::load(super::utils::NAME, super::utils::CONFIG_NAME).expect("Error while reading or creating config file"); // Load config

    let instances = super::utils::get_subdirectories(super::utils::INSTANCES_DIRECTORY);

    print!("{:?}", instances)
}