use proc_macro2::{Ident, TokenStream};
use proc_macro_error::*;
use quote::quote;
use syn::{Data, Fields, Generics};

pub enum MergeableData {
    Enum,
    Newtype,
    Struct(Fields),
}

pub(crate) struct DeriveInput {
    pub ident: Ident,
    pub generics: Generics,
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
            generics: input.generics,
            data,
        }
    }
}

pub(crate) fn generate_merge(input: DeriveInput) -> TokenStream {
    match input.data {
        MergeableData::Enum => generate_merge_enum(input.ident, input.generics),
        MergeableData::Newtype => generate_merge_newtype(input.ident, input.generics),
        MergeableData::Struct(fields) => generate_merge_struct(input.ident, input.generics, fields),
    }
}

fn generate_merge_enum(ident: Ident, generics: Generics) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    quote! {
        impl #impl_generics biome_deserialize::Merge for #ident #ty_generics #where_clause {
            fn merge_with(&mut self, other: Self) {
                *self = other;
            }
        }
    }
}

fn generate_merge_newtype(ident: Ident, generics: Generics) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    quote! {
        impl #impl_generics biome_deserialize::Merge for #ident #ty_generics #where_clause {
            fn merge_with(&mut self, other: Self) {
                self.0 = other.0;
            }
        }
    }
}

fn generate_merge_struct(ident: Ident, generics: Generics, fields: Fields) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let field_idents = fields.into_iter().filter_map(|field| field.ident);
    quote! {
        impl #impl_generics biome_deserialize::Merge for #ident #ty_generics #where_clause {
            fn merge_with(&mut self, other: Self) {
                #( biome_deserialize::Merge::merge_with(&mut self.#field_idents, other.#field_idents); )*
            }
        }
    }
}
