use std::path::Path;

use carmen_db::collections::CollectionExtraction;
use git2::{FetchOptions, build::RepoBuilder};
use walkdir::{DirEntry, WalkDir};

use super::{DocumentFormat, DownloadedDocument, Downloader};

pub struct GitDownloader;

impl Downloader for GitDownloader {
    fn can_download(&self, extraction: &CollectionExtraction) -> bool {
        extraction.source_type == "git"
    }

    async fn download(
        &self,
        extraction: &CollectionExtraction,
        tempdir: &Path,
    ) -> anyhow::Result<Vec<DownloadedDocument>> {
        let mut fetch_opts = FetchOptions::new();
        fetch_opts.depth(1);

        RepoBuilder::new()
            .fetch_options(fetch_opts)
            .clone(&extraction.source, tempdir)?;

        let mut extracted = Vec::new();

        for entry in WalkDir::new(tempdir)
            .into_iter()
            .filter_entry(Self::filter_entry)
            .filter_map(|e| e.ok())
            .filter(Self::filter_file)
        {
            let format = match DocumentFormat::guess_for_path(entry.path()) {
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

            let file_path = entry.into_path();

            extracted.push(DownloadedDocument {
                file_path,
                canonical_path,
                format,
            });
        }

        Ok(extracted)
    }
}

impl GitDownloader {
    fn filter_file(entry: &DirEntry) -> bool {
        if !entry.file_type().is_file() {
            return false;
        }

        if let Some(filename) = entry.path().file_stem().and_then(|stem| stem.to_str()) {
            ![
                "ACKNOWLEDGMENTS",
                "AUTHORS",
                "CODEOWNERS",
                "CODE_OF_CONDUCT",
                "CONTRIBUTING",
                "CONTRIBUTORS",
                "ISSUE_TEMPLATE",
                "LICENSE",
                "PULL_REQUEST_TEMPLATE",
                "SUPPORT",
            ]
            .contains(&filename)
        } else {
            false
        }
    }

    fn filter_entry(entry: &DirEntry) -> bool {
        if let Some(filename) = entry.path().file_name().and_then(|name| name.to_str()) {
            ![".github"].contains(&filename)
        } else {
            false
        }
    }
}
