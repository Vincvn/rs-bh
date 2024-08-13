#[cfg(target_os = "windows")]
use tokio::task;
use crate::process::win32;
use anyhow::Result;
pub async fn kill_by_name(name: &str) -> Vec<Result<()>> {
    let name = name.to_owned();
    if let Ok(result) = task::spawn_blocking(move || {
        win32::kill_by_name(&name)
    }).await {
        return result
    }
    Vec::new()
}

pub async fn kill(pid: u32) -> Result<()>{
    let pid = pid.to_owned();
    if let Ok(result) = task::spawn_blocking(move || {
        win32::kill(pid)
    }).await {
        return result
    }
    Err(anyhow::anyhow!("Failed to kill process: {}", pid))
}

pub async fn pid_by_name(name: &str) -> Vec<u32> {
    let name = name.to_owned();
    if let Ok(result) = task::spawn_blocking(move || {
        win32::pid_by_name(&name)
    }).await {
        return result
    }
    Vec::new()
}