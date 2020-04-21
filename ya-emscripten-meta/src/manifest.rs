use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Manifest {
    /// Deployment id in url like form.
    pub id: String,
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main: Option<EntryPoint>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub entry_points: Vec<EntryPoint>,
    pub runtime: RuntimeType,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mount_points: Vec<MountPoint>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_dir: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct EntryPoint {
    pub id: String,
    pub wasm_path: String,
    #[serde(default)]
    pub args_prefix: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum MountPoint {
    Ro(String),
    Rw(String),
    Wo(String),
}

impl MountPoint {
    pub fn path(&self) -> &str {
        match self {
            MountPoint::Ro(path) => path,
            MountPoint::Rw(path) => path,
            MountPoint::Wo(path) => path,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum RuntimeType {
    Emscripten,
    Wasi,
}

#[cfg(test)]
mod test {
    use crate::manifest::Manifest;

    #[test]
    fn test_manifest() {
        let json = r#"{
            "id": "unlimited.golem.network/wasm-runner/-/test-ls-f324e2a6619979893b7",
            "name": "WASM runner dymamic image for test-ls",
            "entry-points": [ { "id": "test-ls", "wasm-path": "test-ls.wasm" } ],
            "work-dir": "/out",
            "runtime": "emscripten"
        }"#;

        let m: Manifest = serde_json::from_str(json).unwrap();

        eprintln!("m={:?}", m);
    }
}
