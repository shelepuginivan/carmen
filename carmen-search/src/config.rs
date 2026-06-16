use std::env;

pub struct Config {
    pub http_addr: String,
    pub docs_path: Option<String>,
}

impl Config {
    pub fn load_env() -> anyhow::Result<Self> {
        let http_addr = env::var("CARMEN_ADDR").unwrap_or_else(|_| "0.0.0.0:5124".to_owned());
        let docs_path = env::var("CARMEN_DOCS_PATH").ok();

        Ok(Self {
            http_addr,
            docs_path,
        })
    }
}
