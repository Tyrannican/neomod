use std::path::PathBuf;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Neomod {
    pub store: PathBuf,
    pub wow_loc: PathBuf,
}

pub async fn init_neomod() -> Result<Neomod> {
    if let Some(mut target_dir) = dirs::home_dir() {
        target_dir.push(".neomod");

        let mut neomod = Neomod::default();
        neomod.store = target_dir.clone();

        if !target_dir.exists() {
            fs::create_dir(&target_dir).await?;
        }

        return Ok(neomod);
    }

    Err(anyhow!("unable to get HOME path"))
}

#[tokio::main]
async fn main() -> Result<()> {
    Ok(())
}
