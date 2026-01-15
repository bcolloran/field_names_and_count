extern crate proc_macro;

use darling::FromDeriveInput;
use quote::quote;
use syn::DeriveInput;

#[cfg(not(test))]
use syn::parse_macro_input;

mod fields;
mod variants;

fn expand_field_names(input: DeriveInput) -> proc_macro2::TokenStream {
    fields::Receiver::from_derive_input(&input)
        .map(|receiver| quote!(#receiver))
        .unwrap_or_else(|err| err.write_errors())
}

fn expand_variant_names(input: DeriveInput) -> proc_macro2::TokenStream {
    variants::Receiver::from_derive_input(&input)
        .map(|receiver| quote!(#receiver))
        .unwrap_or_else(|err| err.write_errors())
}

#[cfg(not(test))]
#[proc_macro_derive(FieldNames, attributes(field_names))]
pub fn derive_field_names(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand_field_names(parse_macro_input!(input as DeriveInput)).into()
}

#[cfg(not(test))]
#[proc_macro_derive(VariantNames, attributes(variant_names))]
pub fn derive_variant_names(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand_variant_names(parse_macro_input!(input as DeriveInput)).into()
}

#[cfg(test)]
mod tests {
    use super::{expand_field_names, expand_variant_names};
    use pretty_assertions::assert_eq;
    use quote::quote;
    use syn::parse_quote;

    #[test]
    fn derive_field_names_expands() {
        let input = parse_quote! {
            struct Example {
                hello: String,
                world: String,
            }
        };

        let actual = expand_field_names(input);
        let expected = quote! {
            #[automatically_derived]
            impl ::field_names_and_counts::FieldNamesAndCount for Example {
                #[inline]
                fn field_names() -> &'static [&'static str] {
                    &["hello", "world"]
                }

                #[inline]
                fn field_count() -> usize {
                    2usize
                }
            }
        };

        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn derive_variant_names_expands() {
        let input = parse_quote! {
            enum Example {
                Hello(String),
                World { planet: String },
            }
        };

        let actual = expand_variant_names(input);
        let expected = quote! {
            #[automatically_derived]
            impl ::field_names_and_counts::VariantNamesAndCount for Example {
                #[inline]
                fn variant_names() -> &'static [&'static str] {
                    &["Hello", "World"]
                }

                #[inline]
                fn variant_count() -> usize {
                    2usize
                }
            }
        };

        assert_eq!(actual.to_string(), expected.to_string());
    }
}
