//! Tests for FieldNames macro code generation

use field_names_and_counts::{FieldNames, FieldNamesAndCount};

#[derive(FieldNames)]
#[allow(dead_code)]
struct SimpleStruct {
    hello: String,
    world: String,
}

#[test]
fn test_simple_struct() {
    assert_eq!(SimpleStruct::field_names(), &["hello", "world"]);
    assert_eq!(SimpleStruct::field_count(), 2);
}

#[derive(FieldNames)]
#[allow(dead_code)]
struct StructWithSkip {
    hello: String,
    #[field_names(skip)]
    hidden: bool,
    world: String,
}

#[test]
fn test_struct_with_skip() {
    assert_eq!(StructWithSkip::field_names(), &["hello", "world"]);
    assert_eq!(StructWithSkip::field_count(), 2);
}

#[derive(FieldNames)]
#[allow(dead_code)]
struct GenericStruct<T> {
    hello: T,
    world: String,
}

#[test]
fn test_generic_struct() {
    assert_eq!(GenericStruct::<i32>::field_names(), &["hello", "world"]);
    assert_eq!(GenericStruct::<i32>::field_count(), 2);
}

#[derive(FieldNames)]
#[allow(dead_code)]
struct SingleFieldStruct {
    only_field: u64,
}

#[test]
fn test_single_field_struct() {
    assert_eq!(SingleFieldStruct::field_names(), &["only_field"]);
    assert_eq!(SingleFieldStruct::field_count(), 1);
}

#[derive(FieldNames)]
#[allow(dead_code)]
struct ManyFieldsStruct {
    field1: i32,
    field2: i32,
    field3: i32,
    field4: i32,
    field5: i32,
}

#[test]
fn test_many_fields_struct() {
    assert_eq!(
        ManyFieldsStruct::field_names(),
        &["field1", "field2", "field3", "field4", "field5"]
    );
    assert_eq!(ManyFieldsStruct::field_count(), 5);
}

#[derive(FieldNames)]
#[allow(dead_code)]
struct AllSkipped {
    #[field_names(skip)]
    field1: i32,
    #[field_names(skip)]
    field2: i32,
}

#[test]
fn test_all_skipped() {
    let empty_slice: &[&str] = &[];
    assert_eq!(AllSkipped::field_names(), empty_slice);
    assert_eq!(AllSkipped::field_count(), 0);
}
