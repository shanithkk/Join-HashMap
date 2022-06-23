use std::collections::HashMap;

pub fn join_hashmap<T: PartialEq + std::hash::Hash + Eq + Clone, U: Clone, V: Clone>(
    first: HashMap<T, U>,
    second: HashMap<T, V>,
) -> HashMap<T, (U, V)> {
    let mut data: HashMap<T, (U, V)> = HashMap::new();
    for (key, value) in first {
        for (s_key, s_value) in &second {
            if key.clone() == s_key.to_owned() {
                data.insert(key.clone(), (value.clone(), s_value.clone()));
            }
        }
    }
    data
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn join_hashmap_works() {
        let first = HashMap::from([(100, 1500), (101, 2000), (102, 2500)]);

        let second = HashMap::from([
            (100, "hello".to_string()),
            (101, "world".to_string()),
            (102, "good".to_string()),
        ]);
        let datas = join_hashmap(first, second);
        let response = HashMap::from([
            (101, (2000, "world".to_string())),
            (102, (2500, "good".to_string())),
            (100, (1500, "hello".to_string())),
        ]);

        assert_eq!(datas, response);
    }
}
