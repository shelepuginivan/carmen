use std::collections::VecDeque;
use std::sync::LazyLock;
use std::{collections::HashMap, path::Path};

use anyhow::Context;
use carmen_db::extractions::Extraction;
use regex::Regex;
use strum::EnumString;
use tokio::fs;

use crate::document::{Document, DocumentBuilder, DocumentFormat};

use super::Extractor;

// Definitely not the most accurate regular expression for links to other man pages.
static RE_MAN_LINK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(\\fB)?([0-9A-Za-z\._-]+)(\\fR|\\fP)?\(([0-9A-Za-z\._-])\)").unwrap()
});
const RE_CAPTURE_GROUP_NAME: usize = 2;
const RE_CAPTURE_GROUP_SECTION: usize = 4;

#[derive(PartialEq, Eq, Hash)]
struct ManSpec {
    page: String,
    section: String,
}

impl ManSpec {
    fn from_source(src: &str) -> anyhow::Result<Self> {
        let parts = src.split_once('(').context("missing (")?;
        let page = parts.0.to_owned();
        let section = parts.1.strip_suffix(')').context("missing )")?.to_owned();
        Ok(Self { page, section })
    }

    fn canonical_path(&self) -> String {
        format!("{}/{}", self.section, self.page)
    }

    fn filename(&self) -> String {
        format!("{}.{}.raw", self.page, self.section)
    }
}

pub struct ManExtractor;

// TODO: add more providers
#[derive(Default, EnumString)]
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

        let mut processed = HashMap::<ManSpec, Document>::new();
        let mut queue = VecDeque::from([init_frame]);

        // For crawling, do a typical BFS with (possibly) limited depth and length.
        while !queue.is_empty() {
            if processed.len() >= params.crawl_limit as usize {
                break;
            }

            let current = queue.pop_front().unwrap();

            let download_url = params.provider.download_url(&current.spec);
            let content = reqwest::get(download_url).await?.text().await?;
            let document = Self::write_man(tempdir, &current.spec, &content).await?;
            processed.insert(current.spec, document);

            if current.depth == params.crawl_depth {
                continue;
            }

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
                if processed.contains_key(&spec) {
                    continue;
                }

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
