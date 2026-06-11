use std::env;

pub struct Config {
    pub postgres_url: String,
}

impl Config {
    pub fn load_env() -> anyhow::Result<Self> {
        let postgres_url = env::var("POSTGRES_URL")?;

        Ok(Self { postgres_url })
    }
}
