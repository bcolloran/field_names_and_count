//! Tests for VariantNames macro code generation

use field_names_and_counts::{VariantNames, VariantNamesAndCount};

#[derive(VariantNames)]
#[allow(dead_code)]
enum SimpleEnum {
    Hello(String),
    World { planet: String, person: String },
}

#[test]
fn test_simple_enum() {
    assert_eq!(SimpleEnum::variant_names(), &["Hello", "World"]);
    assert_eq!(SimpleEnum::variant_count(), 2);
}

#[derive(VariantNames)]
#[allow(dead_code)]
enum EnumWithSkip {
    Hello(String),
    #[variant_names(skip)]
    Secret(String),
    World {
        planet: String,
        person: String,
    },
}

#[test]
fn test_enum_with_skip() {
    assert_eq!(EnumWithSkip::variant_names(), &["Hello", "World"]);
    assert_eq!(EnumWithSkip::variant_count(), 2);
}

#[derive(VariantNames)]
#[allow(dead_code)]
enum SingleVariantEnum {
    OnlyOne,
}

#[test]
fn test_single_variant_enum() {
    assert_eq!(SingleVariantEnum::variant_names(), &["OnlyOne"]);
    assert_eq!(SingleVariantEnum::variant_count(), 1);
}

#[derive(VariantNames)]
#[allow(dead_code)]
enum ManyVariantsEnum {
    Variant1,
    Variant2,
    Variant3,
    Variant4,
    Variant5,
}

#[test]
fn test_many_variants_enum() {
    assert_eq!(
        ManyVariantsEnum::variant_names(),
        &["Variant1", "Variant2", "Variant3", "Variant4", "Variant5"]
    );
    assert_eq!(ManyVariantsEnum::variant_count(), 5);
}

#[derive(VariantNames)]
#[allow(dead_code)]
enum AllSkipped {
    #[variant_names(skip)]
    Hidden1,
    #[variant_names(skip)]
    Hidden2,
}

#[test]
fn test_all_skipped() {
    let empty_slice: &[&str] = &[];
    assert_eq!(AllSkipped::variant_names(), empty_slice);
    assert_eq!(AllSkipped::variant_count(), 0);
}

#[derive(VariantNames)]
#[allow(dead_code)]
enum MixedVariants {
    UnitVariant,
    TupleVariant(String, i32),
    StructVariant { field1: String, field2: i32 },
}

#[test]
fn test_mixed_variants() {
    assert_eq!(
        MixedVariants::variant_names(),
        &["UnitVariant", "TupleVariant", "StructVariant"]
    );
    assert_eq!(MixedVariants::variant_count(), 3);
}
