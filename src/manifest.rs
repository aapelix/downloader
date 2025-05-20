use std::{collections::HashMap, fs};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::ManifestError;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ManifestAssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: i32,
    pub total_size: i32,
    pub url: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ManifestComponent {
    pub component: String,
    pub major_version: i8,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ManifestFile {
    pub path: Option<String>,
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ManifestDownloads {
    pub client: ManifestFile,
    pub client_mappings: Option<ManifestFile>,
    pub server: ManifestFile,
    pub server_mappings: Option<ManifestFile>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ManifestRule {
    pub action: String,
    pub os: Option<HashMap<String, String>>,
    pub features: Option<HashMap<String, Value>>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ManifestLibraryDownloads {
    pub artifact: Option<ManifestFile>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ManifestLibrary {
    pub downloads: ManifestLibraryDownloads,
    pub name: String,
    pub rules: Option<Vec<ManifestRule>>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct FabricManifestLibrary {
    pub name: String,
    pub url: String,
    pub sha1: Option<String>,
    pub size: Option<u64>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Rules {
    pub action: String,
    pub features: Option<Features>,
    pub os: Option<Os>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Features {
    pub is_demo_user: Option<bool>,
    pub has_custom_resolution: Option<bool>,
    pub is_quick_play_realms: Option<bool>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Os {
    pub arch: Option<String>,
    pub name: Option<String>,
    pub version: Option<String>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Arguments {
    pub game: Vec<JvmArgument>,
    pub jvm: Vec<JvmArgument>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum JvmArgument {
    String(String),
    Struct {
        rules: Vec<Rules>,
        value: serde_json::Value,
    },
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VersionType {
    Release,
    Snapshot,
    OldBeta,
    OldAlpha,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Manifest {
    pub arguments: Arguments,
    pub asset_index: ManifestAssetIndex,
    pub assets: String,
    pub compliance_level: i8,
    pub downloads: ManifestDownloads,
    pub id: String,
    pub java_version: ManifestComponent,
    pub libraries: Vec<ManifestLibrary>,
    pub main_class: String,
    pub minimum_launcher_version: i8,
    pub release_time: String,
    pub time: String,
    #[serde(rename = "type")]
    pub type_: VersionType,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct FabricManifest {
    pub arguments: Arguments,
    pub inherits_from: String,
    pub id: String,
    pub libraries: Vec<FabricManifestLibrary>,
    pub main_class: String,
    pub release_time: String,
    pub time: String,
    #[serde(rename = "type")]
    pub type_: VersionType,
}

fn maven_to_path(coordinate: &str) -> String {
    let parts: Vec<&str> = coordinate.split(':').collect();
    if parts.len() != 3 {
        panic!("Invalid format");
    }
    let group = parts[0].replace('.', "/");
    let artifact = parts[1];
    let version = parts[2];
    format!(
        "{}/{}/{}/{}/{}-{}.jar",
        group, artifact, version, artifact, artifact, version
    )
}

pub fn manifest_from_fabric(
    fabric_manifest: FabricManifest,
    base_manifest: &mut Manifest,
) -> Result<Manifest, ManifestError> {
    let fabric_libraries: Vec<ManifestLibrary> = fabric_manifest
        .libraries
        .into_iter()
        .map(|lib| {
            let path = maven_to_path(&lib.name.clone());
            let sha1 = lib.sha1.unwrap_or_else(|| "".to_string());
            let size = lib.size.unwrap_or(1_i64 as u64);

            ManifestLibrary {
                name: lib.name.clone(),
                downloads: ManifestLibraryDownloads {
                    artifact: Some(ManifestFile {
                        path: Some(path),
                        sha1: sha1,
                        size: size,
                        url: format!("{}{}", lib.url, maven_to_path(&lib.name)),
                    }),
                },
                rules: None,
            }
        })
        .collect();

    let mut combined_libraries = fabric_libraries;
    combined_libraries.extend(base_manifest.libraries.clone());

    let mut combined_game_args = base_manifest.arguments.game.clone();
    combined_game_args.extend(fabric_manifest.arguments.game);

    let mut combined_jvm_args = base_manifest.arguments.jvm.clone();
    combined_jvm_args.extend(fabric_manifest.arguments.jvm);

    Ok(Manifest {
        arguments: Arguments {
            game: combined_game_args,
            jvm: combined_jvm_args,
        },
        libraries: combined_libraries,
        main_class: fabric_manifest.main_class,
        release_time: fabric_manifest.release_time,
        time: fabric_manifest.time,
        type_: fabric_manifest.type_,
        ..base_manifest.clone()
    })
}

pub fn read_manifest_from_str(string: &str) -> Result<Manifest, ManifestError> {
    let manifest: Manifest = serde_json::from_str(string)?;
    Ok(manifest)
}

pub fn read_manifest_from_file(file: &str) -> Result<Manifest, ManifestError> {
    let raw = fs::read_to_string(file)?;
    let manifest: Manifest = read_manifest_from_str(&raw)?;
    Ok(manifest)
}

impl ToString for VersionType {
    fn to_string(&self) -> String {
        match *self {
            VersionType::Release => String::from("Release"),
            VersionType::Snapshot => String::from("Snapshot"),
            VersionType::OldAlpha | VersionType::OldBeta => String::from("Old"),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::VersionType;

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    #[serde(rename_all(deserialize = "camelCase"))]
    struct TestStruct {
        #[serde(rename = "type")]
        type_: VersionType,
    }

    #[test]
    fn version_type_serialize() {
        let st = TestStruct {
            type_: VersionType::Release,
        };
        let expected_json = r#"{"type":"release"}"#;
        let json = serde_json::to_string(&st);

        assert!(json.is_ok());
        assert_eq!(json.unwrap(), expected_json);
    }

    #[test]
    fn version_type_serialize_snake_case() {
        let st = TestStruct {
            type_: VersionType::OldAlpha,
        };
        let expected_json = r#"{"type":"old_alpha"}"#;
        let json = serde_json::to_string(&st);

        assert!(json.is_ok());
        assert_eq!(json.unwrap(), expected_json);
    }

    #[test]
    fn version_type_deserialize() {
        let raw_json = r#"{"type":"old_beta"}"#;
        let expected_st = TestStruct {
            type_: VersionType::OldBeta,
        };
        let json = serde_json::from_str::<TestStruct>(raw_json);

        assert!(json.is_ok());
        assert_eq!(json.unwrap(), expected_st);
    }
}
