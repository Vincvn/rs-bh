pub fn digest(input: &[u8]) -> String {
    let digest = md5::compute(input);
    format!("{:x}", digest)
}
pub fn string(input: &str) -> String {
    digest(&input.as_bytes())
}