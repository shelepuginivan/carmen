use std::env;

pub struct Config {
    pub http_addr: String,
    pub postgres_url: String,
}

impl Config {
    pub fn load_env() -> anyhow::Result<Self> {
        let http_addr = env::var("CARMEN_ADDR").unwrap_or_else(|_| "0.0.0.0:5124".to_owned());
        let postgres_url = env::var("CARMEN_POSTGRES_URL")?;

        Ok(Self {
            http_addr,
            postgres_url,
        })
    }
}
