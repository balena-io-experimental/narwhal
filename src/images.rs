use errors::*;
use network::get;
use serde_json;
use types::Client;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Image {
    id: String,
    parent_id: String,
    repo_tags: Vec<String>,
    repo_digests: serde_json::Value, // TODO: replace with proper type, there are some variances
    created: i64,
    size: i64,
    virtual_size: i64,
    shared_size: i64,
    labels: serde_json::Value, // TODO: replace with proper type, there are some variances
    containers: i64,
}

pub fn get_images_parse(json: &str) -> Result<Vec<Image>> {
    let images: Vec<Image> =
        serde_json::from_str(json).chain_err(|| "Failed to deserialize images response")?;

    Ok(images)
}

pub fn get_images(client: Client) -> Result<Vec<Image>> {
    let response = get(client, "/images/json").chain_err(|| "Failed to get engine version")?;

    if response.status_code != 200 {
        bail!("non-200 response from server");
    }

    let images: Vec<Image> = get_images_parse(&response.body).chain_err(|| "Failed to parse images response")?;

    Ok(images)
}
