use proc_macro2::{Ident, TokenStream};
use proc_macro_error2::*;
use quote::quote;
use syn::{Data, Fields};

pub enum MergeableData {
    Enum,
    Newtype,
    Struct(Fields),
}

pub(crate) struct DeriveInput {
    pub ident: Ident,
    pub data: MergeableData,
}

impl DeriveInput {
    pub fn parse(input: syn::DeriveInput) -> Self {
        let data = match input.data {
            Data::Enum(_) => MergeableData::Enum,
            Data::Struct(data) => {
                if data.fields.iter().all(|field| field.ident.is_some()) {
                    MergeableData::Struct(data.fields)
                } else {
                    MergeableData::Newtype
                }
            }
            Data::Union(_) => abort!(input, "Merge can only be derived for enums and structs"),
        };

        Self {
            ident: input.ident,
            data,
        }
    }
}

pub(crate) fn generate_merge(input: DeriveInput) -> TokenStream {
    match input.data {
        MergeableData::Enum => generate_merge_enum(input.ident),
        MergeableData::Newtype => generate_merge_newtype(input.ident),
        MergeableData::Struct(fields) => generate_merge_struct(input.ident, fields),
    }
}

fn generate_merge_enum(ident: Ident) -> TokenStream {
    quote! {
        impl biome_deserialize::Merge for #ident {
            fn merge_with(&mut self, other: Self) {
                *self = other;
            }
        }
    }
}

fn generate_merge_newtype(ident: Ident) -> TokenStream {
    quote! {
        impl biome_deserialize::Merge for #ident {
            fn merge_with(&mut self, other: Self) {
                biome_deserialize::Merge::merge_with(&mut self.0, other.0);
            }
        }
    }
}

fn generate_merge_struct(ident: Ident, fields: Fields) -> TokenStream {
    let field_idents = fields.into_iter().filter_map(|field| field.ident);
    quote! {
        impl biome_deserialize::Merge for #ident {
            fn merge_with(&mut self, other: Self) {
                #( biome_deserialize::Merge::merge_with(&mut self.#field_idents, other.#field_idents); )*
            }
        }
    }
}
