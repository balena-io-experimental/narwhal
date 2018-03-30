use errors::*;
use types::Client;
use network::get;

use serde_json;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Version {
    pub version: String,
    pub os: String,
    pub kernel_version: String,
    pub go_version: String,
    pub git_commit: String,
    pub arch: String,
    pub api_version: String,
    #[serde(rename(deserialize = "MinAPIVersion"))]
    pub min_api_version: String,
    pub build_time: String,
}

pub fn version(client: Client) -> Result<Version> {
    let response = get(client, "/version").chain_err(|| "Failed to get engine version")?;

    if response.status_code != 200 {
        bail!("non-200 response from server");
    }

    let version: Version =
        serde_json::from_str(&response.body).chain_err(|| "Failed to deserialize engine response")?;

    Ok(version)
}

pub fn ping(client: Client) -> Result<()> {
    let response = get(client, "/_ping").chain_err(|| "Failed to ping engine")?;

    if response.status_code != 200 {
        bail!("non-200 response from engine");
    }

    if response.body != "OK" {
        bail!("Malformed response from engine");
    }

    Ok(())
}
