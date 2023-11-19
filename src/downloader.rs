use reqwest::blocking::get;
use reqwest::blocking::Client;
use std::fs::File;
use std::io::copy;
use std::error::Error;
use serde_json::Value;


pub(crate) fn get_latest(resource_type: &str) -> Result<(), Box<dyn Error>>{
    let req = get("https://launchermeta.mojang.com/mc/game/version_manifest.json")?;
    let versions: Value = serde_json::from_str(&req.text()?)?;
    let latest_version = versions["latest"]["release"].as_str().unwrap();
    match resource_type {
        "vanilla" => get_later_version_vanilla(latest_version),
        "fabric" => get_latest_version_fabric(latest_version),
        "paper" => get_latest_version_paper(latest_version),
        "spigot" => get_latest_version_spigot(latest_version),
        _ => Err("Invalid resource type".into())
    }
}
pub(crate) fn get_version(resource_type: &str, version: &str) -> Result<(), Box<dyn Error>>{
    match resource_type {
        "vanilla" => get_later_version_vanilla(version),
        "fabric" => get_latest_version_fabric(version),
        "paper" => get_latest_version_paper(version),
        "spigot" => get_latest_version_spigot(version),
        _ => Err("Invalid resource type".into())
    }
}
fn get_later_version_vanilla(version: &str) -> Result<(), Box<dyn Error>> {
    let req = get("https://launchermeta.mojang.com/mc/game/version_manifest.json")?;
    let versions: Value = serde_json::from_str(&req.text()?)?;

    let mut ver_meta = String::new();
    for num in versions["versions"].as_array().unwrap() {
        if num["id"].as_str().unwrap() == version {
            ver_meta = num["url"].as_str().unwrap().to_string();
            break;
        }
    }

    let req = get(&ver_meta)?;
    let json: Value = serde_json::from_str(&req.text()?)?;
    let url = json["downloads"]["server"]["url"].as_str().unwrap().to_string();
    download_file(&url, "vanilla-server.jar")
}

fn get_latest_version_fabric(version: &str) -> Result<(), Box<dyn Error>> {
    let req = get(&format!("https://meta.fabricmc.net/v2/versions/loader/{}/", version))?;
    let json: Value = serde_json::from_str(&req.text()?)?;
    let latest_build = json[0]["loader"]["version"].as_str().unwrap().to_string();
    let url = format!("https://maven.fabricmc.net/net/fabricmc/fabric-loader/{}/fabric-loader-{}.jar", latest_build, latest_build);
    download_file(&url, "fabric-server.jar")
}

fn get_latest_version_paper(version: &str) -> Result<(), Box<dyn Error>> {
    let req = get(&format!("https://api.papermc.io/v2/projects/paper/versions/{}/", version))?;
    let json: Value = serde_json::from_str(&req.text()?)?;
    let versions = json["builds"].as_array().unwrap();

    let mut max = 0;
    for num in versions {
        let num = num.as_str().unwrap().parse::<i32>()?;
        if num > max {
            max = num;
        }
    }
    let url = format!("https://papermc.io/api/v2/projects/paper/versions/{}/builds/{}/downloads/paper-{}-{}.jar", version, max, version, max);
    download_file(&url, "paper-server.jar")
}


fn get_latest_version_spigot(version: &str) -> Result<(), Box<dyn Error>> {
    download_file(&format!("https://download.getbukkit.org/spigot/spigot-{}.jar", version), "spigot-server.jar")
}

fn download_file(url: &str, destination: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let mut response = client.get(url).send()?;
    let mut dest = File::create(destination)?;
    copy(&mut response, &mut dest)?;
    Ok(())
}