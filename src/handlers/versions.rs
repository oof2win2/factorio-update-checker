use gotham::handler::HandlerError;
use gotham::mime;
use gotham::prelude::*;
use gotham::state::State;
use hyper::StatusCode;
use reqwest;
use semver;
use serde::Deserialize;
use serde_json;
use std::env;

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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Platform {
    Linux32,
    Linux64,
    LinuxHeadless64,
    Mac,
    Win32,
    Win64,
}
impl Platform {
    fn default() -> Platform {
        Platform::LinuxHeadless64
    }
}

async fn get_available_versions() -> Result<AvailableVersions, reqwest::Error> {
    let req_url = format!(
        "https://updater.factorio.com/get-available-versions?username={}&token={}",
        env::var("FACTORIO_USERNAME").unwrap_or(String::new()),
        env::var("FACTORIO_TOKEN").unwrap_or(String::new())
    );
    let response = reqwest::get(req_url).await.unwrap().text().await.unwrap();
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
            Version::VersionUpdate { from: _, to } => {
                if to > latest_experimental {
                    latest_experimental = to;
                }
            }
            _ => continue,
        }
    }
    latest_experimental
}

#[derive(Deserialize, StateData, StaticResponseExtender, Debug)]
pub struct GetLatestParams {
    #[serde(default = "Platform::default")]
    platform: Platform,
    #[serde(default = "bool::default")]
    experimental: bool,
}
pub async fn get_latest(state: &mut State) -> Result<impl IntoResponse, HandlerError> {
    let query_param = GetLatestParams::borrow_from(state);
    let all_versions = get_available_versions().await.unwrap();
    let versions = match query_param.platform {
        Platform::Linux32 => all_versions.core_linux32,
        Platform::Linux64 => all_versions.core_linux64,
        Platform::LinuxHeadless64 => all_versions.core_linux_headless64,
        Platform::Mac => all_versions.core_mac,
        Platform::Win32 => all_versions.core_win32,
        Platform::Win64 => all_versions.core_win64,
    };
    let latest_version = if query_param.experimental {
        find_latest_experimental(versions.clone())
    } else {
        find_latest_stable(&versions).unwrap().clone()
    };
    Ok((StatusCode::OK, mime::TEXT_PLAIN, latest_version.to_string()))
}
