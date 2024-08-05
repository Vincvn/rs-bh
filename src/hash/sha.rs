use sha2::{Sha256, Sha512, Sha384, Digest};
pub enum Type{
    Sha256,
    Sha384,
    Sha512,
}
pub fn sha256(data: &[u8]) -> String{
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("{:x}", result)
}
pub fn sha512(data: &[u8]) -> String{
    let mut hasher = Sha512::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub fn sha384(data: &[u8]) -> String{
    let mut hasher = Sha384::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("{:x}", result)
}


pub fn string(data: &str, d: Type) -> String {
    let b = data.as_bytes();
    match d {
        Type::Sha256 => sha256(b),
        Type::Sha384 => sha384(b),
        Type::Sha512 => sha512(b),
    }
}