use std::collections::HashSet;
pub fn to_string(d: HashSet<String>) -> String {
    d.into_iter().map(|x| x.to_owned()).collect::<Vec<String>>().join("\n")
}
pub fn json(d: HashSet<String>) -> Option<String> {
    if let Ok(json_string)  = serde_json::to_string(&d) {
        return Some(json_string)
    }
    None
}