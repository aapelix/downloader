use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct LauncherManifestLatest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LauncherManifestVersion {
    pub id: String,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
    pub time: String,
    pub url: String,
    #[serde(rename = "type")]
    pub version_type: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LauncherManifest {
    pub latest: LauncherManifestLatest,
    pub versions: Vec<LauncherManifestVersion>,
}
