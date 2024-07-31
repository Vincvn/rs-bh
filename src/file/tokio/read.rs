use tokio::fs::File;
use anyhow::Result;
use serde::de::DeserializeOwned;
pub async fn bytes(input: &str) -> Result<Vec<u8>> {
    let mut contents : Vec<u8> = Vec::new();
    tokio::io::AsyncReadExt::read_to_end(&mut File::open(input).await?,  &mut contents).await?;
    Ok(contents)
}    
pub async fn string(input: &str)->Result<String> {
    let buffer = bytes(&input).await?;
    let contents = String::from_utf8(buffer)?;
    Ok(contents)
}
pub async fn lines(input: &str)->Result<Vec<String>> {
    Ok(string(&input).await?.lines().into_iter().filter(|s| !s.is_empty()).map(|s|s.to_string()).collect::<Vec<String>>())
}

pub async fn json<T>(input: &str)->Result<T> 
where
    T: DeserializeOwned
{
    let json_string = string(&input).await?;
    Ok(serde_json::from_str::<T>(&json_string)?)
}