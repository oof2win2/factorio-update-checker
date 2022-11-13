use dotenv::dotenv;
use reqwest;
use semver;
use serde::{de, Deserialize};
use serde_json;
use std::{env, fmt};

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum Version {
    VersionUpdate {
        from: semver::Version,
        to: semver::Version,
    },
    StableVersion {
        stable: semver::Version,
    },
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

fn find_latest_stable(versions: &Vec<Version>) -> Option<&semver::Version> {
    for version in versions {
        match version {
            Version::StableVersion { stable } => return Some(stable),
            _ => continue,
        }
    }
    None
}

fn find_latest_experimental(versions: Vec<Version>) -> semver::Version {
    let mut latest_experimental = semver::Version::new(0, 0, 0);
    for version in versions {
        match version {
            Version::VersionUpdate { from, to } => {
                if to > latest_experimental {
                    latest_experimental = to;
                }
            }
            _ => continue,
        }
    }
    latest_experimental
}

fn main() {
    dotenv().ok();
    let versions = get_available_versions().unwrap();
    println!("{:?}", versions.core_linux32);
    println!(
        "{:?}",
        find_latest_stable(&versions.core_linux_headless64).unwrap()
    );
    println!(
        "{:?}",
        find_latest_experimental(versions.core_linux_headless64.clone())
    );
}
