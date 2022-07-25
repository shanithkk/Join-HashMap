mod merge_hashmap;
mod tests;
use merge_hashmap::*;

use std::collections::HashMap;

pub fn join_hashmap<T: PartialEq + std::hash::Hash + Eq + Clone, U: Clone, V: Clone>(
    first: &mut HashMap<T, U>,
    second: &mut HashMap<T, V>,
) -> HashMap<T, (U, V)> {
    let mut data: HashMap<T, (U, V)> = HashMap::new();
    for (key, value) in first {
        for (s_key, s_value) in &mut *second {
            if key.clone() == s_key.to_owned() {
                data.insert(key.clone(), (value.clone(), s_value.clone()));
            }
        }
    }
    data
}
