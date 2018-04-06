use std::collections::HashMap;

use serde_json;

use errors::*;
use network::get;
use types::Client;
use queryparameters::{generate_path, QueryParameters};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Image {
    pub id: String,
    pub parent_id: String,
    pub repo_tags: Option<Vec<String>>,
    pub repo_digests: Option<Vec<String>>,
    pub created: u64,
    pub size: i64,
    pub virtual_size: i64,
    pub shared_size: i64,
    pub labels: Option<HashMap<String, String>>,
    pub containers: i64,
}

pub fn get_images_parse(json: &str) -> Result<Vec<Image>> {
    serde_json::from_str(json).chain_err(|| "Failed to deserialize get_containers response")
}

pub fn get_images(client: Client, args: Option<&mut QueryParameters>) -> Result<Vec<Image>> {
    let path = generate_path("/images/json", args);

    let response = get(client, &path).chain_err(|| "Failed to get images list")?;

    if response.status_code != 200 {
        bail!("non-200 response from server");
    }

    get_images_parse(&response.body)
}
