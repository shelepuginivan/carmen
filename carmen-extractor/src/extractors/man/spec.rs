use std::fmt::{self, Display};

use anyhow::Context;

#[derive(PartialEq, Eq, Hash)]
pub struct ManSpec {
    pub page: String,
    pub section: String,
}

impl ManSpec {
    pub fn from_source(src: &str) -> anyhow::Result<Self> {
        let parts = src.split_once('(').context("missing (")?;
        let page = parts.0.to_owned();
        let section = parts.1.strip_suffix(')').context("missing )")?.to_owned();
        Ok(Self { page, section })
    }

    pub fn canonical_path(&self) -> String {
        format!("{}/{}", self.section, self.page)
    }

    pub fn filename(&self) -> String {
        format!("{}.{}.raw", self.page, self.section)
    }
}

impl Display for ManSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.page, self.section)
    }
}
