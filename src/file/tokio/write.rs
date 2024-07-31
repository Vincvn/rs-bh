use tokio::fs::File;
use anyhow::Result;
use serde::Serialize;
pub async fn bytes(output: &str, data: &[u8]) -> Result<()> {
    Ok(tokio::io::AsyncWriteExt::write_all(&mut File::create(output).await?, data).await?)
}  
pub async fn string(output: &str, data: &String) -> Result<()> {
    bytes(output, data.as_bytes()).await
}

pub async fn lines(output: &str, data: &[String]) -> Result<()> {
    string(output, &data.join("\n")).await
}

pub async fn json<T>(output: &str, data: T) -> Result<()> 
where
    T: Serialize
{
    let json_string = serde_json::to_string(&data)?;
    Ok(string(output, &json_string).await?)
}