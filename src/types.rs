pub struct Client {
    pub socket_path: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct EngineVersion {
    pub version: String,
    pub os: String,
    pub kernel_version: String,
    pub go_version: String,
    pub git_commit: String,
    pub arch: String,
    pub api_version: String,

    // FIXME: min_api_version is actually parsed as min_a_p_i_version by
    // serde. Sort this out somehow, as it's quite an important field
    // pub min_api_version: String,
    pub build_time: String,
}
