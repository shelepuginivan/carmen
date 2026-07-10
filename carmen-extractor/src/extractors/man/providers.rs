use std::{env, time::Duration};

use anyhow::bail;
use enum_dispatch::enum_dispatch;
use log::warn;
use tokio::time::sleep;

use super::spec::ManSpec;

const DOWNLOAD_MAX_RETRIES: u8 = 5;
const DOWNLOAD_BACKOFF_FACTOR: u32 = 2;

#[enum_dispatch]
pub trait Provider {
    async fn get_man_page(&self, spec: &ManSpec) -> anyhow::Result<String>;
}

// TODO: add more providers
#[enum_dispatch(Provider)]
pub enum ManProvider {
    ArchLinux,
}

impl ManProvider {
    pub fn new_from_env() -> Self {
        let v = match env::var("CARMEN_EXTRACTOR_MAN_PROVIDER") {
            Ok(v) => v,
            Err(_) => return ArchLinux.into(),
        };

        match v.as_str() {
            "archlinux" => ArchLinux.into(),
            _ => ArchLinux.into(),
        }
    }
}

pub struct ArchLinux;

impl Provider for ArchLinux {
    async fn get_man_page(&self, spec: &ManSpec) -> anyhow::Result<String> {
        let url = format!(
            "https://man.archlinux.org/man/{}.{}.raw",
            spec.page, spec.section
        );
        download_content(&url).await
    }
}

async fn download_content(url: &str) -> anyhow::Result<String> {
    let mut delay = Duration::from_secs(1);
    let mut attempt = 1;

    let response = loop {
        if let Ok(res) = reqwest::get(url).await {
            break res;
        }

        if attempt >= DOWNLOAD_MAX_RETRIES {
            bail!("Download of {url} failed after {DOWNLOAD_MAX_RETRIES} attempts");
        }

        warn!(
            "Download failed (attempt {attempt}/{DOWNLOAD_MAX_RETRIES}). Retrying in {}s...",
            delay.as_secs()
        );

        sleep(delay).await;
        attempt += 1;
        delay *= DOWNLOAD_BACKOFF_FACTOR;
    };

    Ok(response.text().await?)
}
