use std::path::PathBuf;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use tokio::fs;

type Manifest = Vec<Addon>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Addon {
    pub name: String,
    pub author: String,
    pub description: String,
    pub versions: Vec<String>,
    pub categories: Vec<String>,
    pub source: AddonSource,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddonSource {
    pub git: Option<String>,
    pub zip: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Neomod {
    pub store: PathBuf,
    pub wow_loc: PathBuf,
    pub manifest: Manifest,
    pub installed: Manifest,
}

pub async fn init_neomod() -> Result<Neomod> {
    if let Some(mut target_dir) = dirs::home_dir() {
        target_dir.push(".neomod");

        let mut neomod = Neomod::default();
        neomod.store = target_dir.clone();

        let manifest_str = include_str!("../manifest.json");
        let mut manifest = target_dir.clone();
        manifest.push("manifest.json");

        if !target_dir.exists() {
            fs::create_dir(&target_dir).await?;
            fs::write(&manifest, manifest_str.as_bytes()).await?;
        }

        neomod.manifest = serde_json::from_str(&manifest_str)?;

        return Ok(neomod);
    }

    Err(anyhow!("unable to get HOME path"))
}

#[tokio::main]
async fn main() -> Result<()> {
    let neomod = init_neomod().await?;
    println!("App: {neomod:?}");
    Ok(())
}
