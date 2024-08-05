use anyhow::Result;
pub async fn md5(path: &str) -> Result<String> {
    let bytes = super::read::bytes(path).await?;
    Ok(crate::hash::md5::digest(&bytes))
}

pub async fn sha256(path: &str) -> Result<String> {
    let bytes = super::read::bytes(path).await?;
    Ok(crate::hash::sha::sha256(&bytes))
}