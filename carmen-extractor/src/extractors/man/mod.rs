use std::collections::{HashSet, VecDeque};
use std::sync::LazyLock;
use std::time::Duration;
use std::{collections::HashMap, path::Path};

use anyhow::bail;
use carmen_db::extractions::Extraction;
use log::{info, warn};
use regex::Regex;
use strum::EnumString;
use tokio::fs;
use tokio::time::sleep;

use crate::document::{Document, DocumentBuilder, DocumentFormat};

use super::Extractor;

mod spec;
use spec::ManSpec;

// Definitely not the most accurate regular expression for links to other man pages.
static RE_MAN_LINK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(\\fB)?([0-9A-Za-z\._-]+)(\\fR|\\fP)?\(([0-9A-Za-z\._-])\)").unwrap()
});
const RE_CAPTURE_GROUP_NAME: usize = 2;
const RE_CAPTURE_GROUP_SECTION: usize = 4;

const DOWNLOAD_MAX_RETRIES: u8 = 5;
const DOWNLOAD_BACKOFF_FACTOR: u32 = 2;

pub struct ManExtractor;

// TODO: add more providers
#[derive(Clone, Copy, Default, EnumString)]
enum ManProvider {
    #[default]
    #[strum(serialize = "arch", serialize = "archlinux")]
    ArchLinux,
}

impl ManProvider {
    fn download_url(&self, spec: &ManSpec) -> String {
        match self {
            ManProvider::ArchLinux => format!(
                "https://man.archlinux.org/man/{}.{}.raw",
                spec.page, spec.section
            ),
        }
    }
}

struct ManSourceParams {
    provider: ManProvider,
    crawl_depth: i64,
    crawl_limit: u64,
}

impl ManSourceParams {
    fn from_json(value: &serde_json::Value) -> ManSourceParams {
        let provider = match value["provider"].as_str() {
            Some(v) => v.parse().unwrap_or_default(),
            None => Default::default(),
        };

        let crawl_depth = value["crawl_depth"].as_i64().unwrap_or_default();
        let crawl_limit = value["crawl_limit"].as_u64().unwrap_or(u64::MAX);

        Self {
            provider,
            crawl_depth,
            crawl_limit,
        }
    }
}

struct BFSFrame {
    spec: ManSpec,
    depth: i64,
}

impl Extractor for ManExtractor {
    async fn extract(
        &self,
        extraction: &Extraction,
        tempdir: &Path,
    ) -> anyhow::Result<Vec<Document>> {
        let params = ManSourceParams::from_json(&extraction.parameters);
        let spec = ManSpec::from_source(&extraction.source)?;
        let init_frame = BFSFrame { spec, depth: 0 };

        let mut processed = HashMap::new();
        let mut queue = VecDeque::from([init_frame]);

        // For crawling, do a typical BFS with (possibly) limited depth and length.
        while !queue.is_empty() {
            if processed.len() >= params.crawl_limit as usize {
                break;
            }

            let current = queue.pop_front().unwrap();
            info!("Processing man page {}...", current.spec);

            // Bail if depth == 0, i.e. this is the man page from `source`, warn and continue
            // otherwise.
            let content = match Self::download_man(&current.spec, params.provider).await {
                Ok(s) => s,
                Err(err) if current.depth == 0 => {
                    return Err(err);
                }
                Err(err) => {
                    warn!("Failed to download {}: {err}", &current.spec);
                    continue;
                }
            };

            let document = Self::write_man(tempdir, &current.spec, &content).await?;

            processed.insert(current.spec, document);

            if current.depth == params.crawl_depth {
                continue;
            }

            let mut discovered_links = HashSet::new();

            for link in RE_MAN_LINK.captures_iter(&content) {
                let page = match link.get(RE_CAPTURE_GROUP_NAME) {
                    Some(p) => p.as_str().to_owned(),
                    None => continue,
                };
                let section = match link.get(RE_CAPTURE_GROUP_SECTION) {
                    Some(s) => s.as_str().to_owned(),
                    None => continue,
                };

                let spec = ManSpec { page, section };
                if processed.contains_key(&spec) || discovered_links.contains(&spec) {
                    continue;
                }

                info!("Discovered link to {spec}");
                discovered_links.insert(spec);
            }

            for spec in discovered_links {
                let frame = BFSFrame {
                    spec,
                    depth: current.depth + 1,
                };
                queue.push_back(frame);
            }
        }

        Ok(processed.into_values().collect())
    }
}

impl ManExtractor {
    // TODO: figure out whether it is possible to download many man pages in a single request.
    async fn download_man(spec: &ManSpec, provider: ManProvider) -> anyhow::Result<String> {
        let download_url = provider.download_url(spec);
        let mut delay = Duration::from_secs(1);
        let mut attempt = 1;

        let response = loop {
            if let Ok(res) = reqwest::get(&download_url).await {
                break res;
            }

            if attempt >= DOWNLOAD_MAX_RETRIES {
                bail!("Failed to download {spec} after {DOWNLOAD_MAX_RETRIES} attempts");
            }

            warn!(
                "Failed to download {spec} (attempt {attempt}/{DOWNLOAD_MAX_RETRIES}). Retrying in {}s...",
                delay.as_secs()
            );

            sleep(delay).await;
            attempt += 1;
            delay *= DOWNLOAD_BACKOFF_FACTOR;
        };

        Ok(response.text().await?)
    }

    async fn write_man(prefix: &Path, spec: &ManSpec, content: &str) -> anyhow::Result<Document> {
        let file_path = prefix.join(spec.filename());
        let canonical_path = spec.canonical_path();
        fs::write(&file_path, content).await?;

        DocumentBuilder::default()
            .raw_path(file_path)
            .raw_format(DocumentFormat::Man)
            .canonical_path(canonical_path)
            .build()
            .await
    }
}
