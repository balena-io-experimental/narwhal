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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Plugins {
    pub volume: Option<Vec<String>>,
    pub network: Option<Vec<String>>,
    pub log: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RegistryConfig {
    // TODO: add proper parameters for "IndexConfigs"
    // pub index_configs:
    #[serde(rename(deserialize = " InsecureRegistryCIDRs"))]
    pub insecure_registry_cidrs: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Info {
    pub architecture: Option<String>,
    pub containers: Option<u64>,
    pub containers_running: Option<u64>,
    pub containers_stopped: Option<u64>,
    pub containers_paused: Option<u64>,
    pub cpu_cfs_period: Option<bool>,
    pub cpu_cfs_quota: Option<bool>,
    pub debug: Option<bool>,
    pub discovery_backend: Option<String>,
    pub docker_root_dir: Option<String>,
    pub driver: Option<String>,
    pub driver_status: Option<Vec<Vec<String>>>,
    pub system_status: Option<Vec<Vec<String>>>,
    pub plugins: Option<Plugins>,
    pub experimental_build: Option<bool>,
    pub http_proxy: Option<String>,
    pub https_proxy: Option<String>,
    #[serde(rename(deserialize = "ID"))]
    pub id: Option<String>,
    #[serde(rename(deserialize = "ID"))]
    pub ipv4_forwarding: Option<bool>,
    pub images: Option<u64>,
    pub index_server_address: Option<String>,
    pub init_path: Option<String>,
    pub init_sha1: Option<String>,
    pub kernel_version: Option<String>,
    pub labels: Option<Vec<String>>,
    pub mem_total: Option<u64>,
    pub memory_limit: Option<bool>,
    #[serde(rename(deserialize = "NCPU"))]
    pub ncpu: Option<u64>,
    pub n_events_listener: Option<u64>,
    pub n_fd: Option<u64>,
    pub n_goroutines: Option<u64>,
    pub name: Option<String>,
    pub no_proxy: Option<String>,
    pub oom_kill_disable: Option<bool>,
    #[serde(rename(deserialize = "OSType"))]
    pub os_type: Option<String>,
    pub oom_score_adj: Option<i64>,
    pub operating_system: Option<String>,
    pub registry_config: Option<RegistryConfig>,
    pub swap_limit: Option<bool>,
    pub system_time: Option<String>,
    pub server_version: Option<String>,
}

pub fn version_parse(json: &str) -> Result<Version> {
    serde_json::from_str(json).chain_err(|| "Failed to deserialize version response")
}

pub fn version(client: Client) -> Result<Version> {
    let response = get(client, "/version").chain_err(|| "Failed to get engine version")?;

    if response.status_code != 200 {
        bail!("non-200 response from server");
    }

    version_parse(&response.body)
}

pub fn info_parse(json: &str) -> Result<Info> {
    serde_json::from_str(json).chain_err(|| "Failed to deserialize info response")
}

pub fn info(client: Client) -> Result<Info> {
    let response = get(client, "/info").chain_err(|| "Failed to get engine version")?;

    if response.status_code != 200 {
        bail!("non-200 response from server");
    }

    info_parse(&response.body)
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
