#![allow(dead_code)]

use field_names_and_counts::{VariantNames, VariantNamesAndCount};

#[derive(VariantNames)]
enum Example {
    Hello(String),
    #[variant_names(skip)]
    Secret(String),
    World {
        planet: String,
        person: String,
    },
}

fn main() {
    println!("Variant names: {:?}", Example::variant_names());
    println!("Variant count: {}", Example::variant_count());
}
