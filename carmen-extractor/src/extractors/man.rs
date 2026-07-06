use std::path::Path;

use anyhow::Context;
use carmen_db::extractions::Extraction;
use strum::EnumString;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::document::{Document, DocumentBuilder, DocumentFormat};

use super::Extractor;

pub struct ManExtractor;

// TODO: add more providers
#[derive(Default, EnumString)]
enum ManProvider {
    #[default]
    #[strum(serialize = "arch", serialize = "archlinux")]
    ArchLinux,
}

impl ManProvider {
    fn download_url(&self, page: &str, section: &str) -> String {
        match self {
            ManProvider::ArchLinux => format!("https://man.archlinux.org/man/{page}.{section}.raw"),
        }
    }
}

struct ManSourceParams {
    provider: ManProvider,
}

impl ManSourceParams {
    fn from_json(value: &serde_json::Value) -> ManSourceParams {
        let provider = match value["provider"].as_str() {
            Some(v) => v.parse().unwrap_or_default(),
            None => Default::default(),
        };

        Self { provider }
    }
}

impl Extractor for ManExtractor {
    // TODO: crawl
    async fn extract(
        &self,
        extraction: &Extraction,
        tempdir: &Path,
    ) -> anyhow::Result<Vec<Document>> {
        let params = ManSourceParams::from_json(&extraction.parameters);
        let (page, section) = Self::parse_source(&extraction.source).context("invalid source")?;
        let download_url = params.provider.download_url(page, section);
        let content = reqwest::get(download_url).await?.bytes().await?;

        let canonical_path = format!("{section}/{page}");
        let filename = format!("{page}.{section}.raw");
        let file_path = tempdir.join(filename);
        let mut file = File::create(&file_path).await?;
        file.write_all(&content).await?;

        let document = DocumentBuilder::default()
            .raw_path(file_path)
            .raw_format(DocumentFormat::Man)
            .canonical_path(canonical_path)
            .build()
            .await?;

        Ok(vec![document])
    }
}

impl ManExtractor {
    fn parse_source(source: &str) -> anyhow::Result<(&str, &str)> {
        let (page, section) = source.split_once('(').context("missing (")?;
        let section = section.strip_suffix(')').context("missing )")?;

        Ok((page, section))
    }
}
