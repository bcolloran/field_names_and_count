#![allow(dead_code)]

use field_names_and_counts::{FieldNames, FieldNamesAndCount};

#[derive(FieldNames)]
struct Example<T> {
    hello: T,
    world: String,
}

fn main() {
    println!("Field names: {:?}", Example::<()>::field_names());
    println!("Field count: {}", Example::<()>::field_count());
}
