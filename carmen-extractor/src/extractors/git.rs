use std::path::Path;

use carmen_db::collections::Collection;
use git2::{FetchOptions, build::RepoBuilder};
use walkdir::{DirEntry, WalkDir};

use super::{ExtractedDocument, Extractor};

pub struct GitExtractor;

impl Extractor for GitExtractor {
    fn can_extract(&self, collection: &Collection) -> bool {
        collection.source.as_deref() == Some("git") && collection.url.is_some()
    }

    async fn extract(
        &self,
        collection: &Collection,
        tempdir: &Path,
    ) -> anyhow::Result<Vec<ExtractedDocument>> {
        let mut fo = FetchOptions::new();
        fo.depth(1);

        RepoBuilder::new().fetch_options(fo).clone(
            collection.url.as_ref().expect("collection url must be set"),
            tempdir,
        )?;

        let mut extracted = Vec::new();

        for entry in WalkDir::new(tempdir)
            .into_iter()
            .filter_entry(Self::filter_entry)
            .filter_map(|e| e.ok())
            .filter(Self::filter_file)
        {
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

            extracted.push(ExtractedDocument {
                file_path,
                canonical_path,
            });
        }

        Ok(extracted)
    }
}

impl GitExtractor {
    fn filter_file(entry: &DirEntry) -> bool {
        if !entry.file_type().is_file() {
            return false;
        }

        let accept_stem =
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
            };

        let accept_ext =
            if let Some(extension) = entry.path().extension().and_then(|ext| ext.to_str()) {
                ["md", "txt"].contains(&extension)
            } else {
                false
            };

        accept_stem && accept_ext
    }

    fn filter_entry(entry: &DirEntry) -> bool {
        if let Some(filename) = entry.path().file_name().and_then(|name| name.to_str()) {
            ![".github"].contains(&filename)
        } else {
            false
        }
    }
}
