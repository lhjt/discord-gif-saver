use std::{error::Error, sync::Arc};

use base64::{engine::general_purpose, Engine as _};
use bytes::Bytes;
use decoding::models::settings::Settings;
use parking_lot::Mutex;
use prost::Message;
use sha2::{Digest, Sha256};
use tracing::{error, info, instrument};

use crate::{decoding::models::settings::SavedGif, downloading::DownloadPool};

mod decoding;
mod downloading;

#[instrument(skip_all, fields(gif = %gif.original_reference))]
async fn download_gif(gif: SavedGif) -> Result<(), Box<dyn Error>> {
    let details = match gif.details {
        Some(details) => details,
        None => {
            error!("no details found for gif");
            return Ok(());
        }
    };
    info!("downloading gif");

    let buffer = reqwest::get(details.proxy_address).await?.bytes().await?;
    let kind = infer::get(&buffer);

    let extension = match kind {
        Some(e) => e.extension(),
        None => {
            error!("could not infer extension for gif");
            return Err("no extension found".into());
        }
    };

    let mut hasher = Sha256::new();
    hasher.update(&buffer);
    let digest = format!("{:x}", hasher.finalize());

    let prefix = &digest[0..8];
    let og_filename = match gif.original_reference.split('/').last() {
        Some(f) => f,
        None => {
            error!("could not get filename for gif");
            return Err("no filename found".into());
        }
    };

    let filename = format!("{og_filename}-{prefix}.{extension}");

    tokio::fs::write(format!("output/{filename}"), buffer).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    info!("launching");

    // Read in file and decode the base64 into an array
    let data = tokio::fs::read_to_string("sources/response.txt").await?;

    info!("attempting to parse base64");
    let decoded = general_purpose::STANDARD.decode(data.as_bytes())?;

    // Decode wire format
    info!("attempting to decode");
    let decoded = Settings::decode(Bytes::from(decoded))?;
    // info!("{:#?}", decoded);

    let gifs = match decoded.saved_gifs_wrapper {
        Some(gifs) => gifs,
        None => {
            info!("no gifs found");
            return Ok(());
        }
    };

    let mut results = Vec::with_capacity(gifs.saved_gifs.len());
    let mut dp = DownloadPool::new(gifs.saved_gifs, 20, download_gif);
    dp.complete_tasks(Arc::new(Mutex::new(results))).await;

    Ok(())
}
