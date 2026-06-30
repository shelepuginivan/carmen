use std::{fs::DirEntry, path::Path};

use carmen_db::collections::CollectionExtraction;
use git2::{FetchOptions, build::RepoBuilder};

use crate::document::{Document, DocumentBuilder, DocumentFormat};

use super::Extractor;

pub struct GitHubWikiExtractor;

impl Extractor for GitHubWikiExtractor {
    fn can_extract(&self, extraction: &CollectionExtraction) -> bool {
        extraction.source_type == "github-wiki"
    }

    async fn extract(
        &self,
        extraction: &CollectionExtraction,
        tempdir: &Path,
    ) -> anyhow::Result<Vec<Document>> {
        Self::clone_repo(&extraction.source, tempdir)?;

        let mut extracted = Vec::new();

        for entry in tempdir
            .read_dir()?
            .filter_map(|e| e.ok())
            .filter(Self::filter_file)
        {
            let entry_path = entry.path();

            let format = match DocumentFormat::guess_for_path(&entry_path) {
                Some(f) => f,
                None => continue,
            };

            let canonical_path = match entry
                .path()
                .strip_prefix(tempdir)
                .ok()
                .and_then(|p| p.to_str())
            {
                Some(s) => s.to_owned(),
                None => continue,
            };

            let document = DocumentBuilder::default()
                .raw_path(entry_path)
                .raw_format(format)
                .canonical_path(canonical_path)
                .build()
                .await?;

            extracted.push(document);
        }

        Ok(extracted)
    }
}

impl GitHubWikiExtractor {
    fn clone_repo(repo_url: &str, output: &Path) -> anyhow::Result<()> {
        // TODO: handle more repository url formats.
        let mut fetch_opts = FetchOptions::new();
        fetch_opts.depth(1);

        RepoBuilder::new()
            .fetch_options(fetch_opts)
            .clone(repo_url, output)?;

        Ok(())
    }

    fn filter_file(entry: &DirEntry) -> bool {
        if !entry.file_type().is_ok_and(|t| t.is_file()) {
            return false;
        };

        if let Some(filename) = entry.file_name().to_str() {
            !["_Footer.md", "_Sidebar.md"].contains(&filename)
        } else {
            false
        }
    }
}
