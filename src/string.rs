pub fn split<P: AsRef<str>>(data: &str, index: usize, pat: P) -> Option<String> {
    let parts: Vec<&str> = data.split(pat.as_ref()).collect();
    parts.get(index).map(|&s| s.to_string())
}

pub fn split_to<T: std::str::FromStr, P: AsRef<str>>(data: &str, index: usize, pat: P) -> Option<T> {
    if let Some(part) = split(data, index, pat) {
        T::from_str(&part).ok()
    } else {
        None
    }
}