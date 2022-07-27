# Join-HashMap
This is a small functionality for concatenate two hash-map in rust using their key.
In this package only one functionality is there that is ```join_hashmap``` function it will take two hashmap and based on the key it will join the values. another function are ```merge``` and ```merge_to_string``` these functions combine two hashmap and create new hashmap with HashMap<String, Values>,  and HashMap<String, String>.

## Example

```
// import the library in the crate.
// first hashmap with integer key and integer value

let first = HashMap::from([(100, 1500), (101, 2000), (102, 2500)]);

// second hashmap with integer key and string value

let second = HashMap::from([
    (100, "hello".to_string()),
    (101, "world".to_string()),
    (102, "good".to_string()),
]);

// join the two hashmap using the join_hashmap function

let join_two_hashmap = join_hashmap(first, second);

// expected result of the join_hasmap function

let response = HashMap::from([
    (101, (2000, "world".to_string())),
    (102, (2500, "good".to_string())),
    (100, (1500, "hello".to_string())),
]);

assert_eq!(join_two_hashmap, response);
```

Example for Merge function

```
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
```

More functionalities added in the future for hashmaps

