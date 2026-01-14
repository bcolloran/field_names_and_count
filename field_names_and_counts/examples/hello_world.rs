#![allow(dead_code)]

use field_names_and_counts::{FieldNames, FieldNamesAndCount};

#[derive(FieldNames)]
struct Example {
    hello: String,
    world: String,
    minutes_to_midnight: u32,
    #[field_names(skip)]
    hidden: (),
}

fn main() {
    println!("Field names: {:?}", Example::field_names());
    println!("Field count: {}", Example::field_count());
}
