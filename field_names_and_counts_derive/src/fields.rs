use darling::{FromDeriveInput, FromField, ast::Data};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Generics, Ident};

#[derive(FromDeriveInput)]
#[darling(supports(struct_named))]
pub(crate) struct Receiver {
    ident: Ident,
    generics: Generics,
    data: Data<(), ReceiverField>,
}

impl Receiver {
    pub(crate) fn fields_to_emit(&self) -> Vec<String> {
        self.data
            .as_ref()
            .take_struct()
            .expect("FieldNames only supports named structs")
            .into_iter()
            .filter(|field| !field.skip)
            .map(|field| field.name())
            .collect()
    }
}

impl ToTokens for Receiver {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let fields = self.fields_to_emit();
        let fields_len = fields.len();

        tokens.extend(quote! {
            #[automatically_derived]
            impl #impl_generics ::field_names_and_counts::FieldNamesAndCount for #ident #ty_generics #where_clause {
                #[inline]
                fn field_names() -> &'static [&'static str] {
                    &[#(#fields),*]
                }

                #[inline]
                fn field_count() -> usize {
                    #fields_len
                }
            }
        })
    }
}

#[derive(FromField)]
#[darling(attributes(field_names))]
struct ReceiverField {
    ident: Option<Ident>,
    #[darling(default)]
    skip: bool,
}

impl ReceiverField {
    fn name(&self) -> String {
        self.ident
            .as_ref()
            .expect("FieldNames only supports named fields")
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Receiver;
    use darling::FromDeriveInput;
    use pretty_assertions::assert_eq;
    use quote::{ToTokens, quote};
    use syn::parse_quote;

    #[test]
    fn simple() {
        let input = Receiver::from_derive_input(&parse_quote! {
            #[derive(FieldNames)]
            struct Example {
                hello: String,
                world: String,
            }
        })
        .unwrap();

        assert_eq!(
            input.fields_to_emit(),
            vec!["hello".to_string(), "world".to_string()]
        );
    }

    #[test]
    fn skip_field() {
        let input = Receiver::from_derive_input(&parse_quote! {
            #[derive(FieldNames)]
            struct Example {
                hello: String,
                #[field_names(skip)]
                hidden: bool,
                world: String,
            }
        })
        .unwrap();

        assert_eq!(
            input.fields_to_emit(),
            vec!["hello".to_string(), "world".to_string()]
        );
    }

    #[test]
    fn to_tokens_emits_impl() {
        let input = Receiver::from_derive_input(&parse_quote! {
            #[derive(FieldNames)]
            struct Example {
                hello: String,
                world: String,
            }
        })
        .unwrap();

        let actual = input.to_token_stream();
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
}
