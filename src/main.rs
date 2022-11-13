use std::env;

use dotenv::dotenv;
use reqwest;
use serde::Deserialize;
use serde_json;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Version {
    VersionUpdate { from: String, to: String },
    StableVersion { stable: String },
}
#[derive(Deserialize)]
struct AvailableVersions {
    // #[serde(rename = "core-linux32")]
    #[serde(rename = "core-linux32")]
    core_linux32: Vec<Version>,
    #[serde(rename = "core-linux64")]
    core_linux64: Vec<Version>,
    #[serde(rename = "core-linux_headless64")]
    core_linux_headless64: Vec<Version>,
    #[serde(rename = "core-mac")]
    core_mac: Vec<Version>,
    #[serde(rename = "core-win32")]
    core_win32: Vec<Version>,
    #[serde(rename = "core-win64")]
    core_win64: Vec<Version>,
}

fn get_available_versions() -> Result<AvailableVersions, reqwest::Error> {
    let req_url = format!(
        "https://updater.factorio.com/get-available-versions?username={}&token={}",
        env::var("FACTORIO_USERNAME").unwrap_or(String::new()),
        env::var("FACTORIO_TOKEN").unwrap_or(String::new())
    );
    let response = reqwest::blocking::get(req_url).unwrap().text().unwrap();
    let versions: AvailableVersions = serde_json::from_str(&response).unwrap();
    Ok(versions)
}

fn main() {
    dotenv().ok();
    let versions = get_available_versions().unwrap();
    println!("{:?}", versions.core_linux32);
}
