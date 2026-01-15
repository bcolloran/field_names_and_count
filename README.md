# field_names_and_counts

[![CI](https://github.com/bcolloran/field_names_and_count/actions/workflows/ci.yml/badge.svg)](https://github.com/bcolloran/field_names_and_count/actions/workflows/ci.yml)
[![Codecov](https://codecov.io/gh/bcolloran/field_names_and_count/branch/master/graph/badge.svg)](https://codecov.io/gh/bcolloran/field_names_and_count)

Derive macros and traits for getting field/variant names and counts at compile time.


## Quick examples

### Structs

```rust
use field_names_and_counts::{FieldNames, FieldNamesAndCount};

#[derive(FieldNames)]
struct Person {
    name: String,
    age: u32,
}

assert_eq!(Person::field_names(), &["name", "age"]);
assert_eq!(Person::field_count(), 2);
```

### Enums

```rust
use field_names_and_counts::{VariantNames, VariantNamesAndCount};

#[derive(VariantNames)]
enum Message {
    Ping,
    Data(String),
}

assert_eq!(Message::variant_names(), &["Ping", "Data"]);
assert_eq!(Message::variant_count(), 2);
```

## Installation

```bash
cargo add field_names_and_counts
```

## Attributes

- `#[field_names(skip)]` skips a struct field from the generated list.
- `#[variant_names(skip)]` skips an enum variant from the generated list.

## More examples

### Skipping fields

```rust
use field_names_and_counts::{FieldNames, FieldNamesAndCount};

#[derive(FieldNames)]
struct Config {
    name: String,
    #[field_names(skip)]
    secret: String,
    enabled: bool,
}

assert_eq!(Config::field_names(), &["name", "enabled"]);
assert_eq!(Config::field_count(), 2);
```

### Skipping enum variants

```rust
use field_names_and_counts::{VariantNames, VariantNamesAndCount};

#[derive(VariantNames)]
enum Event {
    Started,
    #[variant_names(skip)]
    Internal(String),
    Finished,
}

assert_eq!(Event::variant_names(), &["Started", "Finished"]);
assert_eq!(Event::variant_count(), 2);
```
