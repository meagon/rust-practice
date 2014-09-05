
// use String::*;

use std::collections::HashMap;

fn main(){

 let mut a = hello();
 println!("{}", a);
   
}

//fn hello<K: String ,V: String>() -> HashMap<K,V> {
fn hello() -> HashMap<String ,String> {

    let mut a= HashMap::new();
    a.insert("hello".to_string(), "world".to_string());
    // return "hello".to_string();
    return a;
}

