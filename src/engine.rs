use errors::*;
use types::{Client, EngineVersion};
use network::get;

use serde_json;

pub fn version(client: Client) -> Result<EngineVersion> {
    let response = get(client, "/version").chain_err(
        || "Failed to get engine version",
    )?;

    if response.status_code != 200 {
        bail!("non-200 response from server");
    }

    let version: EngineVersion = serde_json::from_str(&response.body).chain_err(
        || "Failed to deserialize engine response",
    )?;

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
