mod downloader;

use std::process::{Command, Stdio};
use std::io::Write;
use std::env;
use std::fs;
use std::fs::File;
use std::path;
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
    // Get cli args
    let args: Vec<String> = env::args().collect();

    let main_folder = "~/.minemanager/";

    // Check if the main folder exists
    if !path::Path::new(main_folder).exists() {
        // If it doesn't, create it
        fs::create_dir(main_folder).expect("TODO: panic message");
    }

    let version = "0.1.0";

    // Check if there are any args

    if args.is_empty() {
        // If there are no args, print help
        println!("MineManage v{}", version);
        println!("Usage: mm [command] [args]");
        println!("Commands:");
        println!("    -i --install [instance name] [resource_type] [version] - Install a minecraft server");
        println!("    -s --start [instance name] - Start a minecraft server");
        println!("    -S --stop [instance name]- Stop a minecraft server");
        println!("    -r --restart [instance name] - Restart a minecraft server");
        println!("    -k --kill [instance name] - Kill a minecraft server");
        println!("    -u --update [instance name] - Update a minecraft server");
        println!("    -l --list - List all minecraft servers");
        println!("    -h --help - Print this help message");
        return Ok(());
    }
    // Check if the first arg is a command
    match args[1].as_str() {
        "-i" | "--install" => {
            // Check if there are enough args
            if args.len() < 5 {
                println!("Not enough arguments");
                return Ok(());
            }
            // Check if the instance name is valid
            if args[2].contains("/") || args[2].contains("\\") {
                println!("Invalid instance name");
                return Ok(());
            }
            // Check if the resource type is valid
            match args[3].as_str() {
                "vanilla" | "fabric" | "paper" | "spigot" => {
                    // Check if the version is valid
                    if args[4].contains("/") || args[4].contains("\\") {
                        println!("Invalid version");
                        return Ok(());
                    }
                    // Check if the instance already exists
                    if path::Path::new(&format!("{}/instances/{}", main_folder, args[2])).exists() {
                        println!("Instance already exists");
                        return Ok(());
                    }
                    // Create the instance folder
                    fs::create_dir(&format!("{}/instances/{}", main_folder, args[2])).expect("TODO: panic message");
                    // Download the server jar
                    downloader::get_version(args[3].as_str(), args[4].as_str()).expect("TODO: panic message");
                    // Move the server jar to the instance folder
                    fs::rename(format!("{}-server.jar", args[3]), format!("instances/{}/server.jar", args[2])).expect("TODO: panic message");
                    // Create the config file
                    let mut file = File::create(format!("instances/{}/config.toml", args[2])).expect("TODO: panic message");
                    // Write the config file
                    file.write_all(format!("version = 0\njava_location = \"java\"\nbin_location = \"instances/{}/\"\nbin_name = \"{}-server.jar\"", args[2], args[3].as_str()).as_bytes()).expect("TODO: panic message");
                    // Print success message
                    println!("Instance {} created successfully", args[2]);
                },
                _ => {
                    println!("Invalid resource type");
                    return Ok(());
                }
            }
        },
        "-s" | "--start" => {
            // Check if there are enough args
            if args.len() < 3 {
                println!("Not enough arguments");
                return Ok(());
            }
            // Check if the instance exists
            if !path::Path::new(&format!("instances/{}", args[2])).exists() {
                println!("Instance does not exist");
                return Ok(());
            }
            // start the instance

            let cfg: Config = confy::load("MineManage", format!("instances/{}/config.toml", args[2]).as_str()).expect("TODO: panic message"); // Load config

            let process = Command::new(cfg.java_location)
                .args(["-jar", [cfg.bin_location, cfg.bin_name].join("").as_str()]) // Pass as arguments -jar and the global location of the binary
                .stdin(Stdio::inherit())
                .stdout(Stdio::piped())
                .spawn()
                .expect("java has failed");
        },
        "-S" | "--stop" => {
            // Check if there are enough args
            if args.len() < 3 {
                println!("Not enough arguments");
                return Ok(());
            }
            // Check if the instance exists
            if !path::Path::new(&format!("instances/{}", args[2])).exists() {
                println!("Instance does not exist");
                return Ok(());
            }
            // stop the instance
        },
        "-r" | "--restart" => {
            // Check if there are enough args
            if args.len() < 3 {
                println!("Not enough arguments");
                return Ok(());
            }
            // Check if the instance exists
            if !path::Path::new(&format!("instances/{}", args[2])).exists() {
                println!("Instance does not exist");
                return Ok(());
            }
            // restart the instance
        },
        "-k" | "--kill" => {
            // Check if there are enough args
            if args.len() < 3 {
                println!("Not enough arguments");
                return Ok(());
            }
            // Check if the instance exists
            if !path::Path::new(&format!("instances/{}", args[2])).exists() {
                println!("Instance does not exist");
                return Ok(());
            }
            // kill the instance
        },

        "-u" | "--update" => {
            // Check if there are enough args
            if args.len() < 3 {
                println!("Not enough arguments");
                return Ok(());
            }
            // Check if the instance exists
            if !path::Path::new(&format!("instances/{}", args[2])).exists() {
                println!("Instance does not exist");
                return Ok(());
            }
            // update the instance
        },

        "-l" | "--list" => {
            // Check if there are enough args
            if args.len() < 3 {
                println!("Not enough arguments");
                return Ok(());
            }
            // Check if the instance exists
            if !path::Path::new(&format!("instances/{}", args[2])).exists() {
                println!("Instance does not exist");
                return Ok(());
            }
            // list the instances

            for entry in fs::read_dir("instances").expect("TODO: panic message") {
                let entry = entry.expect("TODO: panic message");
                let binding = entry.path().display().to_string();
                let path = binding.split("/").collect::<Vec<&str>>();
                println!("{}", path[path.len()-2]);

            }

        },

        "-h" | "--help" => {
            // Check if there are enough args
            if args.len() < 3 {
                println!("Not enough arguments");
                return Ok(());
            }
            // Check if the instance exists
            if !path::Path::new(&format!("instances/{}", args[2])).exists() {
                println!("Instance does not exist");
                return Ok(());
            }
            // print help
            println!("MineManage v{}", version);
            println!("Usage: mm [command] [args]");
            println!("Commands:");
            println!("    -i --install [instance name] [resource_type] [version] - Install a minecraft server");
            println!("    -s --start [instance name] - Start a minecraft server");
            println!("    -S --stop [instance name]- Stop a minecraft server");
            println!("    -r --restart [instance name] - Restart a minecraft server");
            println!("    -k --kill [instance name] - Kill a minecraft server");
            println!("    -u --update [instance name] - Update a minecraft server");
            println!("    -l --list - List all minecraft servers");
            println!("    -h --help - Print this help message");

        },
        _ => {
            println!("Invalid command");
            return Ok(());
        }
    }



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


#[cfg(test)]
mod tests {
    use std::path;
    #[test]
    fn path(){
        let binding = path::Path::new("/test/tetas/").display().to_string();
        let path = binding.split("/").collect::<Vec<&str>>();
        // print last value
        println!("{}", path[path.len()-2]);

    }

    #[test]
    fn test() {
        println!("It works!");

    }
}

