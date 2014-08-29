use std::collections::HashMap;


fn main(){ 
let mut map = HashMap::new();

// Insert 10 with key 2
assert_eq!(*map.find_or_insert_with(2i, |&key| 5 * key as uint), 10u);

// Find the existing key
assert_eq!(*map.find_or_insert_with(2, |&key| key as uint), 10);

println!("{}", map);
}
