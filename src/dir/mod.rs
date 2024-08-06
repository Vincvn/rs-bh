use std::fs;
use std::io;
use std::path::Path;
pub mod tokio;
pub fn create_if_not_exists<P: AsRef<Path>>(path: P) -> Result<(), io::Error> {
    let path_ref = path.as_ref();
    if !path_ref.exists() {
        fs::create_dir_all(path_ref)?;
    }
    Ok(())
}