use std::collections::HashSet;

use serde::Serialize;
use rand::seq::SliceRandom;

pub fn to_string(d: &Vec<String>) -> String {
    d.join("\n")
}
pub fn json<T>(d: &Vec<T>) -> Option<String> 
where
    T: Serialize
{
    if let Ok(json_string)  = serde_json::to_string(&d) {
        return Some(json_string)
    }
    None
}

pub fn random<T>(d: &Vec<T>) -> Option<T> 
where
    T: Clone
{
    let mut rng = rand::thread_rng();
    let item = d.choose(&mut rng);
    match item {
        Some(value) => Some(value.clone()),
        None=>None,
    }
}

pub fn taken<T>(d: &Vec<T>, size: usize) -> Vec<T>
where
    T: Clone
{
    d.iter().take(size).map(|f|f.to_owned()).collect()
}

pub fn remove_by_value<T>(d: &mut Vec<T>, value: &T)
where
    T: Eq
{
    if let Some(index ) = d.iter().position(|s| s.eq(value)) {
        d.remove(index);
    }
}

pub fn cut<T>(d: &mut Vec<T>, size: usize) -> Vec<T> 
where
    T: Clone + Eq
{
    let taken: Vec<T> = taken(&d, size);
    taken.clone().into_iter().for_each(|f|{
        remove_by_value(d, &f);
    });
    taken
}

pub fn hashset<T>(vec: Vec<T>) -> HashSet<T>
where
    T: std::hash::Hash + Eq,
{
    vec.into_iter().collect()
}

pub fn merge<T>(source: &mut Vec<T>, merge: Vec<T>) -> Vec<T>
where
    T: Clone
{
    source.extend(merge);
    source.to_owned()
}