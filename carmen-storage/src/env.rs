use std::env;

use crate::error::{Error, Result};

pub fn read_env(key: &'static str) -> Result<String> {
    match env::var(key) {
        Ok(var) => Ok(var),
        Err(_) => Err(Error::Environment(key)),
    }
}
