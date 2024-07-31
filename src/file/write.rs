use std::fs::File;
use anyhow::Result;
use serde::Serialize;
pub fn bytes(output: &str, data: &[u8]) -> Result<()> {
    Ok(std::io::Write::write_all(&mut File::create(output)?, data)?)
}  
pub fn string(output: &str, data: &String) -> Result<()> {
    bytes(output, data.as_bytes())
}

pub fn lines(output: &str, data: &[String]) -> Result<()> {
    bytes(output, data.join("\n").as_bytes())
}

pub fn json<T>(output: &str, data: T) -> Result<()> 
where
    T: Serialize
{
    let json_string = serde_json::to_string(&data)?;
    Ok(string(output, &json_string)?)
}