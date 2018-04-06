use std::collections::HashMap;

use serde_json;

use errors::*;
use network::get;
use types::Client;
use queryparameters::{ generate_path, QueryParameters };

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Port {
    pub private_port: u64,
    pub public_port: u64,
    // We can't use `type` as a field name, so rename
    // this to protocol
    #[serde(rename(deserialize = "Type"))]
    pub protocol: String
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Network {
    #[serde(rename(deserialize = "NetworkID"))]
    pub network_id: String,
    #[serde(rename(deserialize = "EndpointID"))]
    pub endpoint_id: String,
    pub gateway: String,
    #[serde(rename(deserialize = "IPAddress"))]
    pub ip_address: String,
    #[serde(rename(deserialize = "IPPrefixLen"))]
    pub ip_prefix_len: u64,
    #[serde(rename(deserialize = "IPv6Gateway"))]
    pub ipv6_gateway: String,
    #[serde(rename(deserialize = "GlobalIPv6Address"))]
    pub global_ipv6_address: String,
    #[serde(rename(deserialize = "GlobalIPv6PrefixLen"))]
    pub global_ipv6_prefix_len: u64,
    pub mac_address: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkSettings {
    pub networks: HashMap<String, Network>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Mount {
    pub name: String,
    pub source: String,
    pub destination: String,
    pub driver: String,
    pub mode: String,
    #[serde(rename(deserialize = "RW"))]
    pub rw: bool,
    pub propagation: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Container {
    pub id: String,
    pub names: Vec<String>,
    pub image: String,
    #[serde(rename(deserialize = "ImageID"))]
    pub image_id: String,
    pub command: String,
    pub created: u64,
    pub state: String,
    pub status: String,
    pub ports: Vec<Port>,
    pub labels: HashMap<String, String>,
    #[serde(rename(deserialize = "SizeRW"))]
    pub size_rw: Option<u64>,
    pub size_root_fs: Option<u64>,
    pub host_config: HashMap<String, String>,
    pub network_settings: NetworkSettings,
    pub mounts: Vec<Mount>,
}

pub fn get_containers_parse(json: &str) -> Result<Vec<Container>> {
    serde_json::from_str(json).chain_err(|| "Failed to deserialize get_containers response")
}

pub fn get_containers(client: Client, args: Option<&mut QueryParameters>) -> Result<Vec<Container>> {
    let path = generate_path("/containers/json", args);

    let response = get(client, &path)
        .chain_err(|| "Failed to get container list")?;

    if response.status_code != 200 {
        bail!("non-200 response from server");
    }

    get_containers_parse(&response.body)
}
