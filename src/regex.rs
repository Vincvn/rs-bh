use regex::Regex;
use std::collections::HashMap;
use anyhow::Result;
pub fn get_hashmap(html: &String, regex: &str, index1: usize, index2: usize) -> Result<HashMap<String, String>> {
    let re = Regex::new(regex)?;
    let mut elements: HashMap<String, String> = HashMap::new();
    for cap in re.captures_iter(html) {
        let value = cap[index1].to_string();
        let value2 = cap[index2].to_string();
        if !&value.is_empty() && !&value2.is_empty() {
            elements.insert(value, value2);
        }
    }
    Ok(elements)
}

pub fn get(html: &String, regex: &str, index: usize) -> Result<Vec<String>> {
    let re = Regex::new(regex)?;
    let mut elements: Vec<String> = Vec::new();
    for cap in re.captures_iter(html) {
        let value = cap[index].to_string();
        if !&value.is_empty(){
            elements.push(value);
        }
    }
    Ok(elements)
}

pub fn get_string(html: &String, regex: &str, index: usize) -> Result<Option<String>> {
    let elements = get(html, regex, index)?;
    let element = if elements.is_empty() { None }else { 
        let element = elements.iter().next().unwrap();
        Some(element.to_owned())
    };
    Ok(element)
}