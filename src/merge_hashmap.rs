use super::*;
use std::fmt::Display;

pub fn merge<
    T: PartialEq + std::hash::Hash + Eq + Clone + Display,
    U: Clone + Display,
    V: Clone,
>(
    first: &mut HashMap<T, V>,
    second: &mut HashMap<U, V>,
) -> HashMap<String, V> {
    let mut merged = HashMap::new();

    first.iter().for_each(|(k, v)| {
        merged.insert(k.to_string(), v.clone());
    });
    second.iter().for_each(|(k, v)| {
        merged.insert(k.to_string(), v.clone());
    });

    merged
}

pub fn merge_to_string<
    T: PartialEq + std::hash::Hash + Eq + Clone + Display,
    U: Clone + Display,
    V: Clone + Display,
    W: Clone + Display,
>(
    first: &mut HashMap<T, V>,
    second: &mut HashMap<U, W>,
) -> HashMap<String, String> {
    let mut merged = HashMap::new();

    first.iter().for_each(|(k, v)| {
        merged.insert(k.to_string(), v.to_string());
    });
    second.iter().for_each(|(k, v)| {
        merged.insert(k.to_string(), v.to_string());
    });

    merged
}
