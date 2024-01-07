use proc_macro2::{Ident, TokenStream};
use proc_macro_error::*;
use quote::quote;
use syn::{Data, Fields};

pub enum MergeableData {
    Enum,
    NewType,
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
                    MergeableData::NewType
                }
            }
            Data::Union(_) => abort!(input, "Mergeable can only be derived for enums and structs"),
        };

        Self {
            ident: input.ident,
            data,
        }
    }
}

pub(crate) fn generate_mergeable(input: DeriveInput) -> TokenStream {
    match input.data {
        MergeableData::Enum => generate_mergeable_enum(input.ident),
        MergeableData::NewType => generate_mergeable_new_type(input.ident),
        MergeableData::Struct(fields) => generate_mergeable_struct(input.ident, fields),
    }
}

fn generate_mergeable_enum(ident: Ident) -> TokenStream {
    quote! {
        impl biome_deserialize::MergeWith<#ident> for #ident {
            fn merge_with(&mut self, other: #ident) {
                *self = other;
            }
        }
    }
}

fn generate_mergeable_new_type(ident: Ident) -> TokenStream {
    quote! {
        impl biome_deserialize::MergeWith<#ident> for #ident {
            fn merge_with(&mut self, other: #ident) {
                self.0 = other.0;
            }
        }
    }
}

fn generate_mergeable_struct(ident: Ident, fields: Fields) -> TokenStream {
    let merge_fields: Vec<_> = fields
        .iter()
        .filter_map(|field| field.ident.as_ref())
        .map(|field_ident| {
            quote! {
                if let Some(other_value) = other.#field_ident {
                    match self.#field_ident.as_mut() {
                        Some(own_value) => biome_deserialize::MergeWith::merge_with(own_value, other_value),
                        None => {
                            self.#field_ident = Some(other_value);
                        }
                    }
                }
            }
        })
        .collect();

    quote! {
        impl biome_deserialize::MergeWith<#ident> for #ident {
            fn merge_with(&mut self, other: #ident) {
                #( #merge_fields )*
            }
        }
    }
}
