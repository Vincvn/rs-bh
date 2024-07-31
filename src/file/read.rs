use std::fs::File;
use std::io::Result;

use serde::de::DeserializeOwned;
pub fn bytes(input: &str) -> Result<Vec<u8>> {
    let mut contents : Vec<u8> = Vec::new();
    std::io::Read::read_to_end(&mut File::open(input)?, &mut contents)?;
    Ok(contents)
}    
pub fn string(input: &str)->anyhow::Result<String> {
    let buffer = bytes(&input)?;
    let contents = String::from_utf8(buffer)?;
    Ok(contents)
}
pub fn lines(input: &str)->anyhow::Result<Vec<String>> {
    Ok(string(&input)?.lines().into_iter().map(|s|s.to_string()).collect::<Vec<String>>())
}

pub fn json<T>(input: &str)->anyhow::Result<T> 
where
    T: DeserializeOwned
{
    let json_string = string(&input)?;
    match serde_json::from_str::<T>(&json_string){
        Ok(json) => Ok(json),
        Err(e)=>Err(anyhow::anyhow!(e))
    }
}