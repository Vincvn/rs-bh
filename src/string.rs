pub fn split<P: AsRef<str>>(data: &str, index: usize, pat: P) -> Option<String> {
    let parts: Vec<String> = to_vec(data, pat);
    parts.get(index).map(|s|s.to_owned())
}

pub fn split_to<T: std::str::FromStr, P: AsRef<str>>(data: &str, index: usize, pat: P) -> Option<T> {
    if let Some(part) = split(data, index, pat) {
        T::from_str(&part).ok()
    } else {
        None
    }
}

pub fn to_vec<P: AsRef<str>>(data: &str, pat: P) -> Vec<String> {
    data.split(pat.as_ref()).collect::<Vec<&str>>().into_iter().map(|s| s.to_string()).collect::<Vec<String>>()
}