use std::collections::HashMap;

use std::fmt::Result;
use std::io::Result as IoResult;
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Hello, world!");
    let mut map = HashMap::new();
    map.insert(1,2);
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("{}", secret_number);
}
