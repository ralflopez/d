mod error;

pub use self::error::{Error, Result};
use bcrypt::{hash, DEFAULT_COST};

pub fn hash_value(input: &str) -> Result<String> {
    hash(input, DEFAULT_COST).map_err(|e| Error::BcrpytError(e.to_string()))
}

pub fn compare_hash(hash: &str, input: &str) -> Result<()> {
    let val = hash_value(input)?;
    if val != hash {
        return Err(Error::ComparisonError("Inputs are not equal".to_string()));
    }

    Ok(())
}
