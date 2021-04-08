// pub mod serialize;


mod bit_buffer;
pub use bit_buffer::*;

pub mod c;

mod measure;
pub use measure::*;

pub mod id;
pub use id::{Id, IdProvider, IdContext};

mod visitor;
pub use visitor::*;


use std::{collections::hash_map::Entry::{Occupied, Vacant}, hash::Hash};


use std::collections::HashMap;





pub fn union<K:Eq+Hash, V>(a:HashMap<K, V>, b:HashMap<K, V>) -> HashMap<K, V> {
    let mut result = HashMap::new();
    for (k, v) in a {
        result.insert(k, v);
    }

    for (k, v) in b {
        result.insert(k, v);
    }

    result
}


mod stack;
pub use stack::*;
mod template;
pub use template::*;


pub fn insert_empty<T: Eq + Hash, U>(hash_map:&mut HashMap<T, U>, key:T, value:U) {
    match hash_map.entry(key) {
        Occupied(_) => panic!("already inserted"),
        Vacant(entry) => {entry.insert(value);}
    }
}

// people_by_department.entry(&department)
//         .or_insert_with(Vec::new)
//         .push(employee_name);


pub fn concat<T>(mut a:Vec<T>, b:Vec<T>) -> Vec<T> {
    a.extend(b);
    a
}

pub trait FromRef<T> {
    fn from_ref(value:&T) -> Self;
}