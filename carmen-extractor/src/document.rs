use std::path::{Path, PathBuf};

use anyhow::{Context, bail};
use tempfile::NamedTempFile;
use tokio::process::Command;

#[derive(PartialEq, Eq, Default, strum::Display)]
pub enum DocumentFormat {
    #[default]
    #[strum(to_string = "plain")]
    PlainText,
    #[strum(to_string = "gfm")]
    Markdown,
    #[strum(to_string = "rst")]
    ReStructuredText,
}

impl DocumentFormat {
    pub fn guess_for_path(path: &Path) -> Option<Self> {
        let extension = path.extension().and_then(|ext| ext.to_str())?;

        match extension {
            "md" => Some(Self::Markdown),
            "txt" => Some(Self::PlainText),
            "rst" => Some(Self::ReStructuredText),

            _ => None,
        }
    }
}

pub struct Document {
    pub canonical_path: String,
    pub exported_path: PathBuf,
    pub raw_path: PathBuf,
}

#[derive(Default)]
pub struct DocumentBuilder {
    raw_path: Option<PathBuf>,
    raw_format: Option<DocumentFormat>,
    canonical_path: Option<String>,
}

impl DocumentBuilder {
    pub fn raw_path(mut self, path: PathBuf) -> Self {
        self.raw_path = Some(path);
        self
    }

    pub fn raw_format(mut self, format: DocumentFormat) -> Self {
        self.raw_format = Some(format);
        self
    }

    pub fn canonical_path(mut self, path: String) -> Self {
        self.canonical_path = Some(path);
        self
    }

    pub async fn build(self) -> anyhow::Result<Document> {
        let raw_path = self.raw_path.context("raw_path must be set")?;
        let canonical_path = self.canonical_path.context("canonical_path must be set")?;

        let raw_format = self
            .raw_format
            .or_else(|| DocumentFormat::guess_for_path(&raw_path))
            .unwrap_or_default();

        if raw_format == DocumentFormat::Markdown || raw_format == DocumentFormat::PlainText {
            return Ok(Document {
                canonical_path,
                exported_path: raw_path.clone(),
                raw_path,
            });
        }

        let parent = raw_path.parent().context("cannot get parent directory")?;

        let (output, exported_path) = NamedTempFile::new_in(parent)?.keep()?;

        let result = Command::new("pandoc")
            .arg("--standalone")
            .arg("--from")
            .arg(raw_format.to_string())
            .arg("--to")
            .arg("gfm")
            .arg(&raw_path)
            .stdout(output)
            .spawn()?
            .wait()
            .await?;

        if result.success() {
            Ok(Document {
                canonical_path,
                exported_path,
                raw_path,
            })
        } else {
            bail!("failed to convert document with pandoc")
        }
    }
}
