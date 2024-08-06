use std::path::Path;
use anyhow::Result;
use tokio::fs;
pub async fn create_if_not_exists<P: AsRef<Path>>(path: P) -> Result<()> {
    let path_ref = path.as_ref();
    if !path_ref.exists() {
        fs::create_dir_all(path_ref).await?;
    }
    Ok(())
}