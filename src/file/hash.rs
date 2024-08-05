use anyhow::Result;
pub fn md5(path: &str) -> Result<String> {
    let bytes = super::read::bytes(path)?;
    Ok(crate::hash::md5::digest(&bytes))
}

pub fn sha256(path: &str) -> Result<String> {
    let bytes = super::read::bytes(path)?;
    Ok(crate::hash::sha::sha256(&bytes))
}