use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Expr, Fields, Meta, MetaNameValue};

pub(crate) fn derive_throwable(input: DeriveInput) -> syn::Result<TokenStream> {
    let crate_ident = parse_throwable_crate_ident(&input);
    let exception = parse_throwable_attrs(&input)?;
    parse_throwable_input(&input, crate_ident, exception)
}

fn parse_throwable_crate_ident(input: &DeriveInput) -> TokenStream2 {
    let has_throwable_crate = attributes_find_ident(&input.attrs, "throwable_crate");
    let crate_ident = if has_throwable_crate.is_some() {
        quote! { crate }
    } else {
        quote! { phper }
    };
    crate_ident
}

fn parse_throwable_attrs(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let attr = attributes_find_ident(&input.attrs, "throwable_class");
    attr.map(|attr| attr.parse_args::<Expr>().map(|expr| quote! { #expr }))
        .unwrap_or_else(|| Ok(quote! { "Exception" }))
}

fn parse_throwable_input(
    input: &DeriveInput,
    crate_ident: TokenStream2,
    exception: TokenStream2,
) -> syn::Result<TokenStream> {
    let input_ident = &input.ident;

    match &input.data {
        Data::Enum(e) => {
            let mut transparent_idents = Vec::new();

            for variant in &e.variants {
                let attr = attributes_find_ident(&variant.attrs, "throwable");
                match attr {
                    Some(attr) => {
                        if attr.tokens.to_string() != "(transparent)" {
                            return Err(syn::Error::new_spanned(
                                &attr,
                                "now only support #[throwable(transparent)] for variant",
                            ));
                        }
                        match &variant.fields {
                            Fields::Unnamed(f) if f.unnamed.len() == 1 => {
                                transparent_idents.push(variant.ident.clone());
                            }
                            _ => {
                                return Err(syn::Error::new_spanned(
                                    &variant,
                                    "now only support unnamed field with one item mark attribute #[throwable]",
                                ));
                            }
                        }
                    }
                    None => continue,
                }
            }

            let mut class_entry_arms = transparent_idents
                .iter()
                .map(|i| {
                    quote! { Self::#i(e) => #crate_ident::errors::Throwable::class_entry(e), }
                })
                .collect::<Vec<_>>();
            class_entry_arms.push(quote! { _ => ClassEntry::from_globals(#exception).unwrap(), });

            let mut code_arms = transparent_idents
                .iter()
                .map(|i| {
                    quote! { Self::#i(e) => #crate_ident::errors::Throwable::code(e), }
                })
                .collect::<Vec<_>>();
            code_arms.push(quote! { _ => 0, });

            let mut message_arms = transparent_idents
                .iter()
                .map(|i| {
                    quote! { Self::#i(e) => #crate_ident::errors::Throwable::message(e), }
                })
                .collect::<Vec<_>>();
            message_arms.push(quote! { _ => std::string::ToString::to_string(&self), });

            Ok((quote! {
                impl #crate_ident::errors::Throwable for #input_ident {
                    fn class_entry(&self) -> &#crate_ident::classes::StatelessClassEntry {
                        match self {
                            #(#class_entry_arms)*
                        }
                    }

                    fn code(&self) -> u64 {
                        match self {
                            #(#code_arms)*
                        }
                    }

                    fn message(&self) -> std::string::String {
                        match self {
                            #(#message_arms)*
                        }
                    }
                }
            })
            .into())
        }
        Data::Struct(_) => Ok((quote! {
            impl #crate_ident::errors::Throwable for #input_ident {
                fn class_entry(&self) -> &#crate_ident::classes::StatelessClassEntry {
                    ClassEntry::from_globals(#exception).unwrap()
                }
            }
        })
        .into()),
        Data::Union(_) => Err(syn::Error::new_spanned(
            &input,
            "union auto derive Throwable is not supported",
        )),
    }
}

fn attributes_find_ident<'a>(attrs: &'a [Attribute], ident: &'a str) -> Option<&'a Attribute> {
    attrs.iter().find(|attr| attr.path.is_ident(ident))
}
