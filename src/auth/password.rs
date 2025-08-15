use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_password(password: &str, cost: Option<u32>) -> Result<String> {
    let cost = cost.unwrap_or(DEFAULT_COST);
    let hashed = hash(password, cost)?;
    Ok(hashed)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let is_valid = verify(password, hash)?;
    Ok(is_valid)
}