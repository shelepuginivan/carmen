use std::env;
use std::fmt::Display;
use std::str::FromStr;

use crate::Error;

pub fn read_env<T>(key: &'static str) -> Result<Option<T>, Error>
where
    T: FromStr,
    T::Err: Display,
{
    let value = match env::var(key) {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };

    match T::from_str(&value) {
        Ok(v) => Ok(Some(v)),
        Err(err) => Err(Error::InvalidEnvVar(key, err.to_string())),
    }
}
