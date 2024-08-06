use std::{collections:: HashSet, hash::Hash};
use serde::Serialize;
use rand::seq::SliceRandom;
use crate::vec;
pub fn to_string(d: &HashSet<String>) -> String {
    vec::to_string(&d.into_iter().map(|x| x.to_owned()).collect::<Vec<String>>())
}
pub fn json<T>(d: &HashSet<T>) -> Option<String> 
where
    T: Serialize
{
    if let Ok(json_string)  = serde_json::to_string(&d) {
        return Some(json_string)
    }
    None
}
pub fn to_vec<T>(d: &HashSet<T>) -> Vec<T> 
where 
    T:Clone
{
    let vec: Vec<T> = d.iter().cloned().collect();
    vec
}


pub fn random<T>(d: &HashSet<T>) -> Option<T> 
where
    T: Clone
{
    let vec: Vec<T> = to_vec(&d);
    let mut rng = rand::thread_rng();
    let item = vec.choose(&mut rng);
    match item {
        Some(value) => Some(value.clone()),
        None=>None,
    }
}
pub fn remove<T>(d: &mut HashSet<T>, v: &T)
where
    T: Eq + Hash
{
    d.remove(&v);
}
pub fn cut<T>(d: &mut HashSet<T>, size: usize) -> Vec<T> 
where
    T: Clone + Eq + Hash
{
    let taken: Vec<T> = vec::taken(&to_vec(&d), size);
    taken.clone().into_iter().for_each(|f|{
        remove(d, &f);
    });
    taken
}