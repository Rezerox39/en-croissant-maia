#![allow(dead_code)] // engine items are only used on Android builds

use std::{
    fs,
    path::{Path, PathBuf},
};

#[cfg(any(target_os = "android", target_os = "ios"))]
use std::io::Read;

#[cfg(any(target_os = "android", target_os = "ios"))]
use flate2::read::GzDecoder;
use log::info;
use tauri::AppHandle;
#[cfg(any(target_os = "android", target_os = "ios"))]
use tauri::Manager;

use crate::error::Error;

/// Embedded (gzip-compressed) Leela Chess Zero binary for Android (aarch64).
///
/// The placeholder file tracked in the repository is replaced by
/// `scripts/build-lc0-android.sh` before the Android build runs.
const LC0_GZ: &[u8] = include_bytes!("../resources/lc0.gz");

/// The official Maia 1700 neural network weights.
const MAIA_WEIGHTS: &[u8] = include_bytes!("../resources/maia-1700.pb.gz");

const ENGINE_NAME: &str = "Maia 1700";
const ENGINE_ID: &str = "maia-1700";
const WEIGHTS_FILE: &str = "maia-1700.pb.gz";

/// Extracts the bundled Maia 1700 engine (lc0 + weights) into the app's
/// engines directory and registers it in `engines/engines.json` so the GUI
/// lists it as a local engine.
///
/// Only runs on Android; on desktop the user manages engines normally.
pub fn setup(_app: &AppHandle) -> Result<(), Error> {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    return Ok(());

    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        let engines_dir = _app.path().app_data_dir()?.join("engines");
        fs::create_dir_all(&engines_dir)?;

        let lc0_path = engines_dir.join("lc0");
        let weights_path = engines_dir.join(WEIGHTS_FILE);

        if !lc0_path.exists() {
            info!("Extracting bundled lc0 engine binary");
            let mut decoder = GzDecoder::new(LC0_GZ);
            let mut bytes = Vec::new();
            decoder.read_to_end(&mut bytes)?;
            write_executable(&lc0_path, &bytes)?;
        }

        if !weights_path.exists() {
            info!("Extracting bundled Maia 1700 weights");
            fs::write(&weights_path, MAIA_WEIGHTS)?;
        }

        seed_engines_file(&engines_dir, &lc0_path, &weights_path)
    }
}

#[cfg(unix)]
fn write_executable(path: &Path, bytes: &[u8]) -> Result<(), Error> {
    use std::os::unix::fs::PermissionsExt;
    fs::write(path, bytes)?;
    fs::set_permissions(path, fs::Permissions::from_mode(0o755))?;
    Ok(())
}

#[cfg(not(unix))]
fn write_executable(path: &Path, bytes: &[u8]) -> Result<(), Error> {
    fs::write(path, bytes)?;
    Ok(())
}

fn seed_engines_file(
    engines_dir: &Path,
    lc0_path: &PathBuf,
    weights_path: &PathBuf,
) -> Result<(), Error> {
    let engines_file = engines_dir.join("engines.json");

    let mut engines: Vec<serde_json::Value> = match fs::read_to_string(&engines_file) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
        Err(_) => Vec::new(),
    };

    if engines
        .iter()
        .any(|e| e.get("name").and_then(|n| n.as_str()) == Some(ENGINE_NAME))
    {
        return Ok(());
    }

    info!("Registering {} engine in the engines list", ENGINE_NAME);

    engines.push(serde_json::json!({
        "type": "local",
        "id": ENGINE_ID,
        "name": ENGINE_NAME,
        "version": "1.0",
        "path": lc0_path.to_string_lossy().to_string(),
        "image": null,
        "elo": 1700,
        "downloadSize": null,
        "downloadLink": null,
        "go": { "t": "Nodes", "c": 1 },
        "enabled": true,
        "settings": [
            { "name": "WeightsFile", "value": weights_path.to_string_lossy().to_string() },
            { "name": "Threads", "value": 2 },
            { "name": "NNCacheSize", "value": 8 },
            { "name": "MinibatchSize", "value": 1 }
        ]
    }));

    fs::write(&engines_file, serde_json::to_string_pretty(&engines)?)?;
    Ok(())
}
