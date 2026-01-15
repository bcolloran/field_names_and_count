#![doc = include_str!("../../README.md")]

/// Trait for accessing field names and count at compile time.
///
/// This trait is automatically implemented by the `#[derive(FieldNames)]` macro.
pub trait FieldNamesAndCount {
    /// Returns a slice of field names in declaration order.
    fn field_names() -> &'static [&'static str];

    /// Returns the number of fields.
    fn field_count() -> usize;
}

/// Trait for accessing variant names and count at compile time.
///
/// This trait is automatically implemented by the `#[derive(VariantNames)]` macro.
pub trait VariantNamesAndCount {
    /// Returns a slice of variant names in declaration order.
    fn variant_names() -> &'static [&'static str];

    /// Returns the number of variants.
    fn variant_count() -> usize;
}

// Re-export the derive macros for convenience
pub use field_names_and_counts_derive::{FieldNames, VariantNames};
