use std::path::Path;

use carmen_db::collections::CollectionExtraction;
use git2::{FetchOptions, build::RepoBuilder};
use walkdir::{DirEntry, WalkDir};

use crate::document::{Document, DocumentBuilder, DocumentFormat};

use super::Extractor;

pub struct GitExtractor;

struct GitSourceParams<'a> {
    branch: Option<&'a str>,
}

impl<'a> GitSourceParams<'a> {
    fn from_json(value: &'a serde_json::Value) -> GitSourceParams<'a> {
        let branch = value["branch"].as_str();

        Self { branch }
    }
}

impl Extractor for GitExtractor {
    fn can_extract(&self, extraction: &CollectionExtraction) -> bool {
        extraction.source_type == "git"
    }

    async fn extract(
        &self,
        extraction: &CollectionExtraction,
        tempdir: &Path,
    ) -> anyhow::Result<Vec<Document>> {
        let params = GitSourceParams::from_json(&extraction.parameters);
        Self::clone_repo(&extraction.source, &params, tempdir)?;

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

            let document = DocumentBuilder::default()
                .raw_path(file_path)
                .raw_format(format)
                .canonical_path(canonical_path)
                .build()
                .await?;

            extracted.push(document);
        }

        Ok(extracted)
    }
}

impl GitExtractor {
    fn clone_repo(
        repo_url: &str,
        params: &GitSourceParams<'_>,
        output: &Path,
    ) -> anyhow::Result<()> {
        let mut fetch_opts = FetchOptions::new();
        fetch_opts.depth(1);

        let mut builder = RepoBuilder::new();
        builder.fetch_options(fetch_opts);

        if let Some(branch) = params.branch {
            builder.branch(branch);
        }

        builder.clone(repo_url, output)?;
        Ok(())
    }

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
