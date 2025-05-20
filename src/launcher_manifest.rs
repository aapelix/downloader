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

#[derive(Clone, Serialize, Deserialize)]
pub struct FabricVersionManifest {
    pub id: String,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
    pub time: String,
    #[serde(rename = "type")]
    pub version_type: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FabricLoaderInfo {
    pub separator: String,
    pub build: i32,
    pub maven: String,
    pub version: String,
    pub stable: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FabricLoaderManifest {
    pub loader: FabricLoaderInfo,
}
