use std::fs::File;
use std::io::Result;

use serde::Serialize;
pub fn bytes(output: &str, data: &[u8]) -> Result<()> {
    std::io::Write::write_all(&mut File::create(output)?, data)
}  
pub fn string(output: &str, data: &String) -> Result<()> {
    bytes(output, data.as_bytes())
}

pub fn lines(output: &str, data: &[String]) -> Result<()> {
    bytes(output, data.join("\n").as_bytes())
}

pub fn json<T>(output: &str, data: T) -> anyhow::Result<()> 
where
    T: Serialize
{
    let json_string = serde_json::to_string(&data)?;
    match string(output, &json_string){
        Ok(_)=> Ok(()),
        Err(e)=>Err(anyhow::anyhow!(e))
    }
}