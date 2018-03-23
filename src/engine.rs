use errors::*;
use types::{Client, EngineVersion};
use network::simple_get;

use serde_json;

/// Gets the `EngineVersion` for the specified `Client`.
///
/// # Examples
/// ```
/// let client = ::narwhal::types::Client {
///     socket_path: String::from("/var/run/docker.sock"),
/// };
/// let version = ::narwhal::engine::version(client);
/// ```
pub fn version(client: Client) -> Result<EngineVersion> {
    let response = simple_get(client, "/version").chain_err(|| "Failed to get engine version")?;

    if response.status_code != 200 {
        bail!("non-200 response from server");
    }

    let version: EngineVersion =
        serde_json::from_str(&response.body).chain_err(|| "Failed to deserialize engine response")?;

    Ok(version)
}
