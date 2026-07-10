use std::collections::{HashSet, VecDeque};
use std::sync::LazyLock;
use std::{collections::HashMap, path::Path};

use carmen_db::extractions::Extraction;
use log::{info, warn};
use regex::Regex;
use tokio::fs;

use crate::document::{Document, DocumentBuilder, DocumentFormat};

use super::Extractor;

mod providers;
mod spec;

use providers::{ManProvider, Provider};
use spec::ManSpec;

// Definitely not the most accurate regular expression for links to other man pages.
static RE_MAN_LINK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(\\fB)?([0-9A-Za-z\._-]+)(\\fR|\\fP)?\(([0-9A-Za-z\._-])\)").unwrap()
});
const RE_CAPTURE_GROUP_NAME: usize = 2;
const RE_CAPTURE_GROUP_SECTION: usize = 4;

struct ManSourceParams {
    crawl_depth: i64,
    crawl_limit: u64,
}

impl ManSourceParams {
    fn from_json(value: &serde_json::Value) -> ManSourceParams {
        let crawl_depth = value["crawl_depth"].as_i64().unwrap_or_default();
        let crawl_limit = value["crawl_limit"].as_u64().unwrap_or(u64::MAX);

        Self {
            crawl_depth,
            crawl_limit,
        }
    }
}

struct BFSFrame {
    spec: ManSpec,
    depth: i64,
}

pub struct ManExtractor {
    provider: ManProvider,
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
            let content = match self.provider.get_man_page(&current.spec).await {
                Ok(s) => s,
                Err(err) if current.depth == 0 => {
                    return Err(err);
                }
                Err(err) => {
                    warn!("Failed to get man page {}: {err}", &current.spec);
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
    pub fn new_from_env() -> Self {
        let provider = ManProvider::new_from_env();
        Self { provider }
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
