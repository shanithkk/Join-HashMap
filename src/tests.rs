use super::*;

#[test]
fn join_hashmap_works() {
    let mut first = HashMap::from([(100, 1500), (101, 2000), (102, 2500)]);

    let mut second = HashMap::from([
        (100, "hello".to_string()),
        (101, "world".to_string()),
        (102, "good".to_string()),
    ]);
    let datas = join_hashmap(&mut first, &mut second);
    let response = HashMap::from([
        (101, (2000, "world".to_string())),
        (102, (2500, "good".to_string())),
        (100, (1500, "hello".to_string())),
    ]);

    assert_eq!(datas, response);
}

#[test]
fn test_merge() {
    let mut first = HashMap::from([(100, 1500), (101, 2000), (102, 2500)]);
    let mut second = HashMap::from([(106, 5600), (107, 9800), (102, 500)]);

    let datas = merge(&mut first, &mut second);
    let response = HashMap::from([
        ("101".to_string(), 2000),
        ("107".to_string(), 9800),
        ("100".to_string(), 1500),
        ("102".to_string(), 500),
        ("106".to_string(), 5600),
    ]);
    assert_eq!(datas, response);
}

#[test]
fn test_merge_str_key() {
    let mut first = HashMap::from([("a", 97), ("b", 98), ("c", 99)]);
    let mut second = HashMap::from([("d", 100), ("e", 101), ("f", 102)]);

    let datas = merge(&mut first, &mut second);
    let response = HashMap::from([
        ("a".to_string(), 97),
        ("b".to_string(), 98),
        ("c".to_string(), 99),
        ("d".to_string(), 100),
        ("e".to_string(), 101),
        ("f".to_string(), 102),
    ]);
    assert_eq!(datas, response);
}

#[test]
fn test_merge_string_key() {
    let mut first = HashMap::from([
        ("a".to_string(), "97"),
        ("b".to_string(), "98"),
        ("c".to_string(), "99"),
    ]);
    let mut second = HashMap::from([("d", "100"), ("e", "101"), ("f", "102")]);

    let datas = merge(&mut first, &mut second);
    let response = HashMap::from([
        ("a".to_string(), "97"),
        ("b".to_string(), "98"),
        ("c".to_string(), "99"),
        ("d".to_string(), "100"),
        ("e".to_string(), "101"),
        ("f".to_string(), "102"),
    ]);
    assert_eq!(datas, response);
}

#[test]
fn test_merge_different_key() {
    let mut first = HashMap::from([
        ("a".to_string(), "97"),
        ("b".to_string(), "98"),
        ("c".to_string(), "99"),
    ]);
    let mut second = HashMap::from([(100, "100"), (101, "101"), (102, "102")]);

    let datas = merge(&mut first, &mut second);
    let response = HashMap::from([
        ("a".to_string(), "97"),
        ("b".to_string(), "98"),
        ("c".to_string(), "99"),
        ("100".to_string(), "100"),
        ("101".to_string(), "101"),
        ("102".to_string(), "102"),
    ]);
    assert_eq!(datas, response);
}

#[test]
fn test_merge_to_string_different_key() {
    let mut first = HashMap::from([
        ("a".to_string(), "97"),
        ("b".to_string(), "98"),
        ("c".to_string(), "99"),
    ]);
    let mut second = HashMap::from([(100, "100"), (101, "101"), (102, "102")]);

    let datas = merge_to_string(&mut first, &mut second);
    let response = HashMap::from([
        ("a".to_string(), "97".to_string()),
        ("b".to_string(), "98".to_string()),
        ("c".to_string(), "99".to_string()),
        ("100".to_string(), "100".to_string()),
        ("101".to_string(), "101".to_string()),
        ("102".to_string(), "102".to_string()),
    ]);
    assert_eq!(datas, response);
}
