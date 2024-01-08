use proc_macro2::{Ident, TokenStream};
use proc_macro_error::*;
use quote::quote;
use syn::{Data, Fields};

pub(crate) struct DeriveInput {
    pub ident: Ident,
    pub fields: Fields,
}

impl DeriveInput {
    pub fn parse(input: syn::DeriveInput) -> Self {
        let fields = match input.data {
            Data::Struct(data) => data.fields,
            _ => abort!(input, "NoneState can only be derived for structs"),
        };

        Self {
            ident: input.ident,
            fields,
        }
    }
}

pub(crate) fn generate_none_state(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    let fields: Vec<_> = input
        .fields
        .iter()
        .filter_map(|field| field.ident.as_ref())
        .collect();

    quote! {
        impl biome_deserialize::NoneState for #ident {
            fn none() -> Self {
                Self {
                    #( #fields: None ),*
                }
            }
        }
    }
}
