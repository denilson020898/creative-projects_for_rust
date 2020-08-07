use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref DICTIONARY: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        // m.insert(11, "foo");
        // m.insert(12, "bar");
        m.entry(11).or_insert("foo");
        m.entry(12).or_insert("bar");
        println!("Initialized!");
        m
    };
}

fn main() {
    println!("Started");
    println!("DICTIONARY contains {:?}", *DICTIONARY);
    println!("DICTIONARY contains {:?}", *DICTIONARY);
}
