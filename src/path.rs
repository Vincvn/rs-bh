use std::path::PathBuf;
use anyhow::Result;
pub struct CurrentDir {
    path : Option<String>
}
impl CurrentDir {
    pub fn new() -> Self {
        CurrentDir {
            path : None,
        }
    }
    pub fn set(&mut self, path: &str) -> &mut Self {
        self.path = Some(path.to_owned());
        self
    }
    pub fn path(&self) -> Result<PathBuf> {
        let current_dir = std::env::current_dir()?;
        let path = match self.path.to_owned() {
            Some(p) => current_dir.join(p),
            None => current_dir,
        };
        Ok(path)
    }
    pub fn to_string(&self) -> Result<String> {
        Ok(self.path()?.to_string_lossy().into_owned())
    }

}