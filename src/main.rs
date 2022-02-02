use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;
use std::fmt::Display;

fn main() {
    let mut schema = Model::<String, i32>::new("test_schema", 60);

    println!("Name: {}", schema.name);
    println!("Time to live: {}", schema.ttl);
    
    schema.put("key1".to_string(), 69);

    let data = schema.get(&"key1".to_string());
    println!("Data: {:?}", data.unwrap());
}

pub struct Model<K, V> {
    name: &'static str,
    ttl: u64,
    data: HashMap<K, V>
}

impl<K, V> Model<K, V> where K: Eq + Hash + Display, V: Display {
    pub fn new(name: &'static str, ttl: u64) -> Model<K, V> {
        Model {
            name,
            ttl,
            data: HashMap::<K, V>::new()
        }
    }
    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        self.data.insert(key, value)
    }
    pub fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }
}